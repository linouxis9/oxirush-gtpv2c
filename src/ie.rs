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

//! The information element of TS 29.274 §8.2.1, the IEs that a typed scope keeps as
//! octets, and the order of a scope on the wire.

use alloc::vec::Vec;

use crate::Error;
use oxirush_utils::fields::Octets;

/// What a type of the crate is as an IE: its IE type and its name in Table 8.1-1.
/// The registry writes it for each type.
pub trait IeType {
    /// The IE type.
    const IE_TYPE: u16;
    /// Its name in Table 8.1-1.
    const TYPE_NAME: &'static str;
    /// Its length, when Table 8.1-1 fixes it.
    const FIXED: Option<u16> = None;
}

/// How a value is read from the octets of its IE, after the Length, and written back.
pub trait Codec: Sized {
    /// The value of the octets, or `None` for octets that are not one: the scope keeps
    /// them as they are.
    fn decode(octets: &[u8]) -> Option<Self>;

    /// Write the value.
    ///
    /// # Errors
    ///
    /// A value whose fields disagree with what its flags say, as its clause has them.
    fn encode(&self, out: &mut Vec<u8>) -> Result<(), Error>;

    /// How many octets [`encode`](Self::encode) writes.
    fn encoded_len(&self) -> usize;
}

/// The value of a row of a table, as the scope holds and checks it: what a grouped IE
/// does as such, and a value that is not one by default.
pub(crate) trait Listed: Codec + crate::check::Rules {
    /// Decode into the field of a row that is there once: `false`, and the field left
    /// empty, for octets that are no value. A grouped IE decodes in its place.
    #[inline]
    fn decode_into(slot: &mut Option<Self>, octets: &[u8]) -> bool {
        *slot = Self::decode(octets);
        slot.is_some()
    }

    /// Decode at the end of the field of a row that is a list.
    fn decode_push(list: &mut Vec<Self>, octets: &[u8]) -> bool;

    /// The octets that a value takes beyond its own size: those of the boxes, the lists
    /// and the kept IEs of a grouped IE, which a few octets of the wire make, and not the
    /// octets of its values, which are those of the wire.
    #[cfg(feature = "endpoint")]
    fn held(&self) -> usize {
        0
    }

    /// The check of a value in its scope, of the context of its row: a grouped IE checks
    /// its table there.
    fn check_in(
        &self,
        path: &crate::check::Path<'_>,
        context: crate::check::Context,
        found: &mut Vec<crate::check::Deviation>,
    ) {
        let _ = context;
        self.check(path, found);
    }

    /// Whether its octets read as the value: of a value that writes some, and of a
    /// grouped IE as [`Scope::plain`](oxirush_utils::tables::Scope::plain) has it.
    fn plain(&self) -> bool {
        self.encoded_len() > 0
    }
}

/// What a value that is not a grouped IE writes as its [`Listed`].
macro_rules! listed_value {
    ($($ty:ident),* $(,)?) => {$(
        impl crate::ie::Listed for $ty {
            #[inline]
            fn decode_push(list: &mut alloc::vec::Vec<Self>, octets: &[u8]) -> bool {
                match <Self as crate::ie::Codec>::decode(octets) {
                    Some(value) => {
                        list.push(value);
                        true
                    }
                    None => false,
                }
            }
        }
    )*};
}
pub(crate) use listed_value;

/// An IE that a scope keeps as its octets, in its place: one whose type and instance the
/// table of the scope does not have (§7.7.9), or that no release has, one whose octet 4 has
/// spare bits that are set, one of a null length, one whose value does not decode as its
/// type, and one that repeats an IE that the table has once (§7.7.10). `Debug` does not
/// print the octets of a key.
#[derive(Clone, PartialEq, Eq)]
pub struct RawIe {
    /// The IE type.
    pub ie_type: u16,
    /// Octet 4 of the IE: the instance in bits 4 to 1, and the spare bits 8 to 5.
    pub instance: u8,
    /// A Length to write in place of the one of the octets: a false length, which a test
    /// sends on purpose. A decoded IE never has one.
    pub length: Option<u16>,
    /// The content after the first four octets.
    pub octets: Octets,
}

/// Whether an IE of `ie_type` may hold a key: an MM Context (types 103 to 108), with its
/// keys and its vectors, or a grouped IE that a table gives one, at any depth.
pub(crate) fn holds_secret(ie_type: u16) -> bool {
    if (103..=108).contains(&ie_type) {
        return true;
    }
    let Some(info) = crate::registry::ie_type_info(ie_type) else {
        return false;
    };
    let mut rows = info.tables.iter().flat_map(|table| table.rows.iter());
    rows.any(|row| row.ie_type.0 != ie_type && holds_secret(row.ie_type.0))
}

impl core::fmt::Debug for RawIe {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        let mut debug = f.debug_struct("RawIe");
        debug
            .field("ie_type", &self.ie_type)
            .field("instance", &self.instance)
            .field("length", &self.length);
        match holds_secret(self.ie_type) {
            true => debug.field("octets", &format_args!("{} octets", self.octets.len())),
            false => debug.field("octets", &self.octets),
        };
        debug.finish()
    }
}

impl RawIe {
    /// The IE of `ie_type` and `instance` whose content is `octets`, whatever they are:
    /// none for an IE of a null length.
    pub fn new(ie_type: u16, instance: u8, octets: impl Into<Octets>) -> Self {
        Self {
            ie_type,
            instance,
            length: None,
            octets: octets.into(),
        }
    }

    /// Whether the IE has a null length.
    pub fn is_null(&self) -> bool {
        self.octets.is_empty()
    }

    /// Whether its Length is that of its content: one that is not does not frame, nor do
    /// the IEs after it (§7.7.7).
    pub(crate) fn frames(&self) -> bool {
        self.length
            .is_none_or(|length| usize::from(length) == self.octets.len())
    }

    /// Write the IE.
    pub(crate) fn encode(&self, out: &mut Vec<u8>) -> Result<(), Error> {
        let at = begin((self.ie_type, self.instance), out)?;
        out.extend_from_slice(&self.octets);
        finish(self.ie_type, at, self.length, out)
    }
}

/// An entry of the `order` of a scope, the order of the wire, which a decoded message keeps
/// when its IEs are not in the order of their tables, so that it encodes back byte for
/// byte: the next value of a field of an IE type, or the next IE of the scope's `raw`.
pub type Placed = oxirush_utils::tables::Placed<(u16, u8)>;

pub(crate) use oxirush_utils::tables::Placing;

/// What the tables of oxirush-utils keep of an IE as octets.
impl oxirush_utils::tables::Raw for RawIe {
    type Key = (u16, u8);
    type Error = Error;

    #[inline]
    fn key(&self) -> (u16, u8) {
        (self.ie_type, self.instance)
    }

    #[inline]
    fn encode(&self, out: &mut Vec<u8>) -> Result<(), Error> {
        RawIe::encode(self, out)
    }

    #[inline]
    fn encoded_len(&self) -> usize {
        4 + self.octets.len()
    }

    #[inline]
    fn content(&self) -> &[u8] {
        &self.octets
    }
}

/// Why the IEs of a scope do not decode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IeFault {
    /// Fewer octets than the four of an IE header are left (§7.6.7).
    HeaderCut {
        /// How many there are.
        octets: usize,
    },
    /// The Length of an IE runs past its scope (§7.6.7).
    Length {
        /// The IE type.
        ie_type: u16,
        /// Its Length.
        length: u16,
        /// The octets that the scope has after the IE header.
        available: usize,
    },
}

impl core::fmt::Display for IeFault {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        match self {
            Self::HeaderCut { octets } => write!(
                f,
                "{octets} octets are left, fewer than the four of an IE header"
            ),
            Self::Length {
                ie_type,
                length,
                available,
            } => {
                write!(
                    f,
                    "IE type {ie_type} has the Length {length}, and {available} octets follow its header"
                )
            }
        }
    }
}

/// The raw IE of a key, its type and its octet 4, and its content.
pub(crate) fn raw((ie_type, instance): (u16, u8), body: &[u8]) -> RawIe {
    RawIe {
        ie_type,
        instance,
        length: None,
        octets: Octets::copied(body),
    }
}

/// Start an IE of a key, its type and its octet 4: where its Length goes. A type that
/// octet 1 cannot say is refused: one of 256 and more goes after type 254 (§8.2.1A),
/// which a scope keeps as octets.
pub(crate) fn begin((ie_type, instance): (u16, u8), out: &mut Vec<u8>) -> Result<usize, Error> {
    let Ok(octet) = u8::try_from(ie_type) else {
        return Err(Error::HeaderField {
            field: "IE Type",
            value: ie_type.into(),
        });
    };
    out.extend_from_slice(&[octet, 0, 0, instance]);
    Ok(out.len() - 3)
}

/// Write the Length of the IE that starts at `at`: `length`, or that of its content.
pub(crate) fn finish(
    ie_type: u16,
    at: usize,
    length: Option<u16>,
    out: &mut [u8],
) -> Result<(), Error> {
    let written = out.len() - at - 3;
    let length = match length {
        Some(length) => length,
        None => u16::try_from(written).map_err(|_| Error::TooLong {
            ie_type,
            length: written,
        })?,
    };
    out[at..at + 2].copy_from_slice(&length.to_be_bytes());
    Ok(())
}

/// Write the IE of a value, of the key of its row.
pub(crate) fn encode_value<T: IeType + Codec>(
    key: (u16, u8),
    value: &T,
    out: &mut Vec<u8>,
) -> Result<(), Error> {
    let at = begin(key, out)?;
    value.encode(out)?;
    finish(key.0, at, None, out)
}
