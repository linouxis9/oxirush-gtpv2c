/*
   OxiRush
   Copyright 2026 Valentin D'Emmanuele

   Licensed under the Apache License, Version 2.0 (the "License");
   you may not use this file except in compliance with the License.
   You may obtain a copy of the License at

   http://www.apache.org/licenses/LICENSE-2.0

   Unless required by applicable law or agreed to in writing, software
   distributed under the License is distributed on an "AS IS" BASIS,
   WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
   See the License for the specific language governing permissions and
   limitations under the License.
*/

//! A GTPv2-C message: its header and its typed body, and the datagram that carries one
//! with a piggybacked one (§5.5.1).

use alloc::vec::Vec;

use crate::Error;
use crate::check::{Context, Deviation, DeviationKind};
use crate::enums::CauseValue;
use crate::header::Header;
use crate::macros::gtpv2c_messages;

include!("message_types.rs");

/// The name of the message, its TEID and its sequence number:
///
/// ```
/// # use oxirush_gtpv2c::*;
/// let echo = Message {
///     header: Header::path(7),
///     body: EchoRequest::new(Recovery::new(1)).into(),
/// };
/// assert_eq!(echo.to_string(), "Echo Request (sequence 7)");
/// ```
impl core::fmt::Display for Message {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self.body.name() {
            Some(name) => f.write_str(name)?,
            None => write!(f, "GTPv2-C message type {}", self.body.message_type())?,
        }
        match self.header.teid {
            Some(teid) => write!(f, " (TEID {teid:#x}, sequence {})", self.header.sequence),
            None => write!(f, " (sequence {})", self.header.sequence),
        }
    }
}

/// A message type as Table 6.1-1 has it, with the table of its IEs; `name()` is the name
/// that the table gives it.
pub type MessageTypeInfo = oxirush_utils::tables::MessageTypeInfo<crate::Gtpv2c>;

/// The message type of `message_type`.
pub fn message_type_info(message_type: u8) -> Option<&'static MessageTypeInfo> {
    MESSAGE_TYPES
        .iter()
        .find(|info| info.message_type == message_type)
}

/// Whether a message type is one of path management or of Version Not Supported
/// Indication, whose header has no TEID (§5.4): Echo Request, Echo Response and Version
/// Not Supported Indication.
pub(crate) fn is_path(message_type: u8) -> bool {
    matches!(message_type, 1..=3)
}

/// A GTPv2-C message.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Message {
    /// Its header.
    pub header: Header,
    /// Its message type and its IEs.
    pub body: Body,
}

impl Message {
    /// A message of path management of `body` and `sequence`, without a TEID.
    pub fn path(sequence: u32, body: impl Into<Body>) -> Self {
        Self {
            header: Header::path(sequence),
            body: body.into(),
        }
    }

    /// An EPC specific message of `body` and `sequence`, to the peer's TEID `teid`.
    pub fn tunnel(teid: u32, sequence: u32, body: impl Into<Body>) -> Self {
        Self {
            header: Header::tunnel(teid, sequence),
            body: body.into(),
        }
    }

    /// The Cause of the message at its level, of instance 0, whichever message it is.
    pub fn cause(&self) -> Option<CauseValue> {
        self.body.cause()
    }

    /// The restart counter of the Recovery of the message, of instance 0, whichever
    /// message it is (§8.5).
    pub fn recovery(&self) -> Option<u8> {
        self.body.recovery()
    }

    /// Decode one message, which `octets` are whole.
    ///
    /// # Errors
    ///
    /// Octets shorter than a header, a Message Length that is not the length of the
    /// octets (§7.7.3), and IEs whose framing runs past the message (§7.7.7). The error
    /// has the header, which the answer of a receiver takes.
    pub fn decode(octets: &[u8]) -> Result<Self, Error> {
        let (message, used) = Self::decode_first(octets)?;
        if used != octets.len() {
            let (header, message_type, length) = Header::decode(octets).ok_or(Error::TooShort {
                octets: octets.len(),
            })?;
            return Err(Error::MessageLength {
                header,
                message_type,
                length,
                available: octets.len() - 4,
            });
        }
        Ok(message)
    }

    /// Decode each message of a datagram: the message, and the one that its P flag says
    /// is piggybacked after it (§5.5.1), each on its own. A message that does not decode
    /// is the error of [`decode`](Self::decode) in its place. The message without the P
    /// flag is the rest of the datagram, and so is one whose P flag says that another
    /// follows where the datagram ends.
    pub fn decode_datagram(octets: &[u8]) -> Vec<Result<Self, Error>> {
        datagram_parts(octets)
            .into_iter()
            .map(Message::decode)
            .collect()
    }

    fn decode_first(octets: &[u8]) -> Result<(Self, usize), Error> {
        let (header, message_type, length) = Header::decode(octets).ok_or(Error::TooShort {
            octets: octets.len(),
        })?;
        let end = 4 + usize::from(length);
        let Some(content) = octets.get(header.encoded_len()..end) else {
            return Err(Error::MessageLength {
                header,
                message_type,
                length,
                available: octets.len() - 4,
            });
        };
        let body = Body::decode(message_type, content).map_err(|fault| Error::Ies {
            header,
            message_type,
            fault,
        })?;
        Ok((Self { header, body }, end))
    }

    /// Encodes the message.
    ///
    /// `encode()` is permissive on purpose, so that a test tool can send malformed
    /// messages: it writes spare bits, reserved values, IEs that the table of the message
    /// does not have, and a false IE Length ([`RawIe::length`](crate::RawIe::length)) or
    /// Message Length ([`Header::length`]) as they are. [`check`](Self::check) reports
    /// each of them.
    ///
    /// # Errors
    ///
    /// Only a value that it cannot write so that it decodes back to the same value:
    /// [`Error::Inconsistent`], [`Error::FieldLength`], [`Error::HeaderField`],
    /// [`Error::TooLong`] and [`Error::MessageTooLong`].
    pub fn encode(&self) -> Result<Vec<u8>, Error> {
        let mut out = Vec::new();
        self.encode_into(&mut out)?;
        Ok(out)
    }

    /// Appends the message to `out`, as [`encode`](Self::encode) writes it, with one
    /// reservation of its length. After an error, `out` is as it was.
    ///
    /// # Errors
    ///
    /// Those of [`encode`](Self::encode).
    pub fn encode_into(&self, out: &mut Vec<u8>) -> Result<(), Error> {
        self.encode_with(false, out)
    }

    /// Appends a datagram of `messages` to `out`: each but the last with its P flag set,
    /// so that the next is piggybacked on it (§5.5.1).
    ///
    /// # Errors
    ///
    /// Those of [`encode`](Self::encode).
    pub fn encode_datagram(messages: &[Message], out: &mut Vec<u8>) -> Result<(), Error> {
        let start = out.len();
        for (at, message) in messages.iter().enumerate() {
            if let Err(error) = message.encode_with(at + 1 < messages.len(), out) {
                out.truncate(start);
                return Err(error);
            }
        }
        Ok(())
    }

    fn encode_with(&self, piggybacked: bool, out: &mut Vec<u8>) -> Result<(), Error> {
        let start = out.len();
        let header = self.header.encoded_len();
        let expected = self.body.encoded_len();
        out.reserve(header + expected);
        out.resize(start + header, 0);
        let written = self.body.encode(out).and_then(|()| {
            let body = out.len() - start - header;
            debug_assert_eq!(
                body, expected,
                "the encoded length of the IEs is the one computed first"
            );
            self.header.encode_into(
                self.body.message_type(),
                body,
                piggybacked,
                &mut out[start..],
            )
        });
        if written.is_err() {
            out.truncate(start);
        }
        written
    }

    /// Validates the message against TS 29.274 V19.6.0 and returns every deviation that
    /// it finds. Each [`Deviation`] has the path of the IE in the tree of the message, the
    /// clause that is broken, the kind of deviation and, for an initial message, the Cause
    /// with which its receiver answers it (§7.7).
    ///
    /// The message is checked as its receiver reads it. An IE that is stored as raw
    /// octets is checked as the value of its row, so `check()` returns the same result
    /// before and after `Message::decode(m.encode())`.
    ///
    /// # What is checked
    ///
    /// - **Header** (§5.5.1): the version, the T flag of the message type, the MP flag
    ///   without a TEID, the spare bits and the Message Length (§7.7.3).
    /// - **IE tables of clause 7**, by IE type and instance, for the message and every
    ///   grouped IE: missing mandatory IEs (§7.7.6), IEs of an instance or a type that
    ///   the table does not have (§7.7.9), repeated IEs (§7.7.10), the Length of each IE
    ///   (§7.7.7), the fixed lengths of Table 8.1-1, IEs that do not decode (§7.7.8), the
    ///   IEs that a rejection may carry (§6.1.1), and IE types and message types that
    ///   Release 19 does not define.
    /// - **Values of clause 8** that the crate lays out: spare bits, reserved and spare
    ///   values.
    ///
    /// # What is not checked
    ///
    /// A message without deviations is not proven conformant: the conditions of the C
    /// rows, which are in words; the content of the values that the crate keeps as
    /// octets; the state of the peer, its TEIDs and its sequence numbers; and the
    /// procedures of clause 7 (retransmission, the timers and the error indications).
    pub fn check(&self) -> Vec<Deviation> {
        let message_type = self.body.message_type();
        let mut found = Vec::new();
        let context = Context {
            request: answered(message_type),
            ..Context::message()
        };
        let read = self.body.as_read(context, &mut found).map(|body| Self {
            header: self.header,
            body,
        });
        found.extend(read.as_ref().unwrap_or(self).check_read());
        found
    }

    /// The check of a message as it is.
    fn check_read(&self) -> Vec<Deviation> {
        let message_type = self.body.message_type();
        // A Failure Indication has a rejection Cause alone, and its other IEs follow their
        // rows (§6.1.1).
        let failure = matches!(message_type, 65 | 67 | 69 | 70);
        let rejection = self
            .body
            .cause()
            .filter(|cause| cause.rejects() && !failure);
        let context = Context {
            request: answered(message_type),
            rejected: rejection,
            ..Context::message()
        };
        let mut found = self.header_deviations(context);
        // A mandatory row of "MM Context" is one row per type (errata.tsv): one of them is
        // there. The answer names the EPS one, of N26 and S10.
        if self.body.lacks_mm_context() {
            let mut deviation = Deviation::new(
                "/gtpv2c",
                "7.7.6",
                DeviationKind::MandatoryMissing { ie: (107, 0) },
            );
            deviation.cause = context.cause(true);
            found.push(deviation);
        }
        self.body.check(
            &crate::check::Path::Root(crate::check::ROOT),
            context,
            &mut found,
        );
        found
    }

    /// The deviations of the header (§5.5.1), each a rule: whether it is broken, where,
    /// the clause and what.
    fn header_deviations(&self, context: Context) -> Vec<Deviation> {
        use DeviationKind::Rule;
        let header = &self.header;
        let message_type = self.body.message_type();
        let known = !matches!(self.body, Body::Other { .. });
        let path = known && is_path(message_type);
        let tunnel = known && !path;
        let wide = header.overflow().map(|(field, _)| match field {
            "Version" => ("/header/Version", "a version of more than 3 bits"),
            "Spare" => (
                "/header/Spare",
                "spare bits of octet 1 of more than their bits",
            ),
            "Sequence Number" => (
                "/header/Sequence Number",
                "a sequence number of more than 24 bits",
            ),
            "Message Priority" => (
                "/header/Message Priority",
                "a message priority of more than 4 bits",
            ),
            _ => (
                "/header/Spare octet",
                "spare bits of the last octet of more than its bits",
            ),
        });
        let (wide_field, too_wide) = wide.unwrap_or_default();
        #[rustfmt::skip]
        let rules = [
            (header.version != 2, "/header/Version", "5.5.1", Rule("the version is not 2")),
            (wide.is_some(), wide_field, "5.5.1", Rule(too_wide)),
            (header.spare != 0, "/header/Spare", "5.5.1", DeviationKind::SpareBits),
            (header.spare_last != 0, "/header/Spare octet", "5.5.1", DeviationKind::SpareBits),
            (path && header.teid.is_some(), "/header/TEID", "5.5.1", Rule("a message of path management has the T flag")),
            (tunnel && header.teid.is_none(), "/header/TEID", "5.5.1", Rule("an EPC specific message has no TEID")),
            (path && header.piggybacked, "/header/P", "5.5.1", Rule("a message of path management has the P flag")),
        ];
        let broken = rules.into_iter().filter(|(broken, ..)| *broken);
        let mut found: Vec<Deviation> = broken
            .map(|(_, path, clause, kind)| Deviation::new(path, clause, kind))
            .collect();
        // A Message Length that is not the length of the octets after the first four: the
        // receiver discards the message, or answers "Invalid Length" (§7.7.3).
        let after = header.encoded_len() - 4 + self.body.encoded_len();
        if header
            .length
            .is_some_and(|length| usize::from(length) != after)
        {
            let rule = Rule("a Message Length other than the octets after the first four");
            let mut deviation = Deviation::new("/header/Message Length", "7.7.3", rule);
            deviation.cause = context.invalid_length();
            found.push(deviation);
        }
        found
    }
}

/// Whether the receiver of a message type answers its deviations with a Cause: an initial
/// message but an Echo Request, which has none (§7.1.1).
fn answered(message_type: u8) -> bool {
    is_initial(message_type) && message_type != 1
}

/// The octets of each message of a datagram, as [`Message::decode_datagram`] frames
/// them, before they are decoded.
pub(crate) fn datagram_parts(mut octets: &[u8]) -> Vec<&[u8]> {
    let mut messages = Vec::new();
    loop {
        let Some((header, _, length)) = Header::decode(octets) else {
            messages.push(octets);
            return messages;
        };
        let end = 4 + usize::from(length);
        let whole = header.piggybacked && end < octets.len() && end >= header.encoded_len();
        let this = if whole { &octets[..end] } else { octets };
        messages.push(this);
        if !whole {
            return messages;
        }
        octets = &octets[end..];
    }
}

#[cfg(feature = "tree")]
pub(crate) mod forms {
    use oxirush_utils::tree::{self, Members, Reading, Refused, Tree, Value};

    use super::{Body, Message};
    use crate::header::Header;

    /// The members that the tree of a message may have.
    pub(crate) const NAMES: &[&str] = &["message", "header", "ies"];

    /// The members that the header of the tree of a message may have: the fields of the
    /// header (§5.5.1), and the Message Length to write in place of the message's.
    pub(crate) const HEADER: &[&str] = &[
        "Version",
        "P",
        "Spare",
        "TEID",
        "Sequence Number",
        "Message Priority",
        "Spare octet",
        "Message Length",
    ];

    /// `{"message": NAME, "header": {"TEID": ..., "Sequence Number": ...}, "ies": {...}}`,
    /// with the other fields of the header where they are not what a sender writes.
    impl Tree for Message {
        const NAME: &'static str = "GTPv2-C message";

        fn to_tree(&self) -> Value {
            let mut members = Members::new();
            members.put(
                "message",
                match self.body.name() {
                    Some(name) => tree::text(name),
                    None => tree::unsigned(self.body.message_type().into()),
                },
            );
            members.put("header", header_tree(&self.header));
            members.put("ies", self.body.ies_tree());
            members.finish()
        }

        fn from_tree(value: &Value, reading: &mut Reading) -> Result<Self, Refused> {
            let read = reading.members(value, NAMES)?;
            let kind =
                reading.member(&read, "message", |value, reading| match value.is_number() {
                    true => {
                        let code = reading.unsigned(value, 0, 255)? as u8;
                        match super::message_type_info(code) {
                            Some(info) => Err(reading.refuse(
                                tree::ErrorKind::Value,
                                alloc::format!(
                                    "message type {code} is the {}, which goes by its name",
                                    info.name()
                                ),
                            )),
                            None => Ok(Err(code)),
                        }
                    }
                    false => Ok(Ok(reading.named(value, &Body::names())?)),
                })?;
            let header = reading.member(&read, "header", read_header)?;
            let body = reading.member(&read, "ies", |value, reading| {
                Body::from_ies_tree(kind, value, reading)
            })?;
            Ok(Self { header, body })
        }
    }

    /// The tree of a header: the TEID that it has and its sequence number, and its other
    /// fields where they are not what a sender writes.
    fn header_tree(header: &Header) -> Value {
        let mut members = Members::new();
        if header.version != 2 {
            members.put("Version", tree::unsigned(header.version.into()));
        }
        if header.piggybacked {
            members.put("P", Value::Bool(true));
        }
        if header.spare != 0 {
            members.put("Spare", tree::unsigned(header.spare.into()));
        }
        members.put_optional("TEID", header.teid.map(|teid| tree::unsigned(teid.into())));
        members.put("Sequence Number", tree::unsigned(header.sequence.into()));
        members.put_optional(
            "Message Priority",
            header
                .priority
                .map(|priority| tree::unsigned(priority.into())),
        );
        if header.spare_last != 0 {
            members.put("Spare octet", tree::unsigned(header.spare_last.into()));
        }
        members.put_optional(
            "Message Length",
            header.length.map(|length| tree::unsigned(length.into())),
        );
        members.finish()
    }

    /// The header that the tree of a header writes.
    fn read_header(value: &Value, reading: &mut Reading) -> Result<Header, Refused> {
        let read = reading.members(value, HEADER)?;
        let small = |reading: &mut Reading, name: &'static str, most: u64| {
            reading.optional(&read, name, |value, reading| {
                reading.unsigned(value, 0, most)
            })
        };
        let version = small(reading, "Version", 7)?.unwrap_or(2) as u8;
        let piggybacked = reading
            .optional(&read, "P", |value, reading| reading.boolean(value))?
            .unwrap_or(false);
        let teid = reading.optional(&read, "TEID", |value, reading| {
            reading.unsigned(value, 0, u32::MAX.into())
        })?;
        let spare = small(reading, "Spare", if teid.is_some() { 3 } else { 7 })?.unwrap_or(0) as u8;
        let sequence = reading.member(&read, "Sequence Number", |value, reading| {
            reading.unsigned(value, 0, 0xff_ffff)
        })? as u32;
        let priority = small(reading, "Message Priority", 15)?.map(|priority| priority as u8);
        let spare_last = small(
            reading,
            "Spare octet",
            if priority.is_some() { 15 } else { 255 },
        )?
        .unwrap_or(0) as u8;
        let length = small(reading, "Message Length", 65535)?.map(|length| length as u16);
        Ok(Header {
            version,
            piggybacked,
            teid: teid.map(|teid| teid as u32),
            sequence,
            priority,
            spare,
            spare_last,
            length,
        })
    }

    impl Message {
        /// The tree of the message.
        pub fn to_tree(&self) -> Value {
            Tree::to_tree(self)
        }

        /// The message that a tree writes.
        ///
        /// # Errors
        ///
        /// Where the tree is not a message, at the path of the place that it refuses,
        /// which [`paths::select`](crate::paths::select) takes, and why.
        pub fn from_tree(value: &Value) -> Result<Message, tree::Error> {
            tree::from_value(value).map_err(|error| {
                let said = alloc::format!("this is not a value of {}: ", <Self as Tree>::NAME);
                let why = error
                    .reason()
                    .strip_prefix(&said[..])
                    .unwrap_or(error.reason());
                let error = tree::Error::new(error.kind(), error.at(), why);
                crate::paths::located(value, error)
            })
        }
    }
}
