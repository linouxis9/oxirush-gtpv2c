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

//! The GTPv2-C header (TS 29.274 clause 5): its flags, the TEID of an EPC message, the
//! sequence number and the message priority.

/// The header of a message, without its message type, which is the message's.
///
/// What a sender sets to zero and a receiver ignores is kept as it is received, so that a
/// message encodes back to its octets: `spare` holds the spare bits of octet 1, and
/// `spare_last` the bits of the last octet that the priority does not have.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Header {
    /// The version, bits 8 to 6 of octet 1: 2 (§5.5.1).
    pub version: u8,
    /// The P flag: a piggybacked message follows this one in the datagram (§5.5.1).
    /// [`Message::encode_datagram`](crate::Message::encode_datagram) sets it on each
    /// message but the last.
    pub piggybacked: bool,
    /// The TEID, after the T flag: an EPC specific message has one, Echo and Version Not
    /// Supported Indication none (§5.3, §5.4).
    pub teid: Option<u32>,
    /// The sequence number, of 24 bits.
    pub sequence: u32,
    /// The message priority, after the MP flag of a header with a TEID: 0, the highest, to
    /// 15 (§5.5.1).
    pub priority: Option<u8>,
    /// The spare bits of octet 1: bits 2 and 1 with a TEID, bits 3 to 1 without one.
    pub spare: u8,
    /// The bits of the last octet of the header that the priority does not have.
    pub spare_last: u8,
    /// A Message Length to write in place of the one of the message: a false length,
    /// which a test sends on purpose. A decoded message never has one.
    pub length: Option<u16>,
}

impl Header {
    /// The header of a message of path management, without a TEID.
    pub const fn path(sequence: u32) -> Self {
        Self {
            version: 2,
            piggybacked: false,
            teid: None,
            sequence,
            priority: None,
            spare: 0,
            spare_last: 0,
            length: None,
        }
    }

    /// The header of an EPC specific message, to the peer's TEID (0 where the sender
    /// does not have it, §5.5.2).
    pub const fn tunnel(teid: u32, sequence: u32) -> Self {
        Self {
            teid: Some(teid),
            ..Self::path(sequence)
        }
    }

    /// How many octets the header has on the wire: 8, or 12 with a TEID.
    pub fn encoded_len(&self) -> usize {
        match self.teid {
            Some(_) => 12,
            None => 8,
        }
    }

    /// The field of the header whose value its bits cannot hold, and the value.
    pub(crate) fn overflow(&self) -> Option<(&'static str, u64)> {
        let fields = [
            ("Version", u64::from(self.version), 7),
            ("Sequence Number", u64::from(self.sequence), 0x00ff_ffff),
            ("Message Priority", self.priority.map_or(0, u64::from), 15),
            (
                "Spare",
                u64::from(self.spare),
                if self.teid.is_some() { 3 } else { 7 },
            ),
            (
                "Spare octet",
                u64::from(self.spare_last),
                if self.priority.is_some() { 15 } else { 255 },
            ),
        ];
        fields
            .into_iter()
            .find(|(_, value, most)| value > most)
            .map(|(field, value, _)| (field, value))
    }

    /// Write the header of a message of `message_type` whose IEs have `body` octets, with
    /// its P flag `piggybacked`, at the start of `out`, which has room for it.
    pub(crate) fn encode_into(
        &self,
        message_type: u8,
        body: usize,
        piggybacked: bool,
        out: &mut [u8],
    ) -> Result<(), crate::Error> {
        if let Some((field, value)) = self.overflow() {
            return Err(crate::Error::HeaderField { field, value });
        }
        // The octets after the first four: the TEID and the sequence number with them.
        let counted = self.encoded_len() - 4 + body;
        let length = match self.length {
            Some(length) => length,
            None => u16::try_from(counted).map_err(|_| crate::Error::MessageTooLong {
                message_type,
                length: counted,
            })?,
        };
        let flags = (self.version << 5)
            | u8::from(piggybacked || self.piggybacked) << 4
            | u8::from(self.teid.is_some()) << 3
            | u8::from(self.priority.is_some() && self.teid.is_some()) << 2
            | self.spare;
        out[0] = flags;
        out[1] = message_type;
        out[2..4].copy_from_slice(&length.to_be_bytes());
        let mut at = 4;
        if let Some(teid) = self.teid {
            out[at..at + 4].copy_from_slice(&teid.to_be_bytes());
            at += 4;
        }
        out[at..at + 3].copy_from_slice(&self.sequence.to_be_bytes()[1..]);
        out[at + 3] = match self.priority {
            Some(priority) => priority << 4 | self.spare_last,
            None => self.spare_last,
        };
        Ok(())
    }

    /// The header, the message type and the Message Length of the first octets of a
    /// message: `None` for fewer octets than its header.
    pub fn decode(octets: &[u8]) -> Option<(Self, u8, u16)> {
        let (&flags, rest) = octets.split_first()?;
        let teid = flags & 0x08 != 0;
        let header = if teid { 12 } else { 8 };
        if octets.len() < header {
            return None;
        }
        let message_type = rest[0];
        let length = u16::from_be_bytes([octets[2], octets[3]]);
        let (teid, at) = match teid {
            true => (
                Some(u32::from_be_bytes([
                    octets[4], octets[5], octets[6], octets[7],
                ])),
                8,
            ),
            false => (None, 4),
        };
        let sequence = u32::from_be_bytes([0, octets[at], octets[at + 1], octets[at + 2]]);
        let last = octets[at + 3];
        let priority = (teid.is_some() && flags & 0x04 != 0).then_some(last >> 4);
        let spare_mask = if teid.is_some() { 0x03 } else { 0x07 };
        Some((
            Self {
                version: flags >> 5,
                piggybacked: flags & 0x10 != 0,
                teid,
                sequence,
                priority,
                spare: flags & spare_mask,
                spare_last: match priority {
                    Some(_) => last & 0x0f,
                    None => last,
                },
                length: None,
            },
            message_type,
            length,
        ))
    }
}
