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

//! GTPv2-C (TS 29.274 V19.6.0): the messages of S11, S5/S8, S3, S10, S16 and N26, their
//! IEs, the tables of clause 7 that say which IEs each message and grouped IE has, and a
//! check of a message against them.
//!
//! The facts of the specification are tables of macros: one line per IE type (Table
//! 8.1-1), one row per IE of a table of clause 7, one layout per value. The codec, the
//! check, the trees and the paths are written from them by oxirush-utils.
//!
//! A message decodes and encodes byte-exact: an IE that its table does not have, of an
//! instance that it does not have, with spare bits set, or that does not decode as its
//! type, is kept as its octets in its place.

#![no_std]
#![deny(unsafe_code)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(not(feature = "tree"), allow(rustdoc::broken_intra_doc_links))]

extern crate alloc;
#[cfg(test)]
extern crate std;

mod check;
mod enums;
mod fields;
pub mod framing;
mod header;
mod ie;
mod macros;
mod message;
mod mm_context;
#[cfg(feature = "tree")]
#[cfg_attr(docsrs, doc(cfg(feature = "tree")))]
pub mod paths;
mod registry;
mod table;
mod uli;
mod values;
mod values_hand;

use alloc::string::{String, ToString};
use core::fmt;

pub use check::{Check, Deviation, DeviationKind};
pub use enums::*;
pub use fields::*;
pub use header::Header;
pub use ie::{Codec, IeFault, IeType, Placed, RawIe};
pub use message::{Body, MESSAGE_TYPES, Message, MessageTypeInfo, is_initial, message_type_info};
pub use mm_context::{
    AccessRestriction, Later, MmContextEpsSecurityContextQuadrupletsAndQuintuplets,
    OldSecurityContext, Quadruplet, Quintuplet,
};
pub use oxirush_utils::fields::Octets;
pub use registry::*;
pub use table::{Interfaces, On, Presence, Repeat, Role, Row, Table};
pub use uli::{Location, Uli};
pub use values::*;
pub use values_hand::{Cause, FqCsid, NodeId, OffendingIe, Paa};

/// The trees of oxirush-utils, of the version that this crate is built with: a message
/// and each IE as JSON whose members keep the order of the specification,
/// [`tree::Value`], and `tree!`, which writes one in the code as `serde_json::json!` does.
/// [`paths`] reads and edits them by path.
#[cfg(feature = "tree")]
#[cfg_attr(docsrs, doc(cfg(feature = "tree")))]
pub use oxirush_utils::tree;

/// The examples of the README, which the documentation tests compile and run: each
/// checks what it gives with `assert_eq!`.
#[cfg(all(doctest, feature = "tree"))]
#[doc = include_str!("../README.md")]
struct Readme;

/// GTPv2-C as the tables of oxirush-utils take it: an IE is keyed by its IE type and its
/// octet 4, the instance with the spare bits, framed as §8.2.1 has it. One whose spare
/// bits are set, of a null length, or a Private Extension is kept as octets.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Gtpv2c;

impl oxirush_utils::tables::Protocol for Gtpv2c {
    type Key = (u16, u8);
    type Raw = RawIe;
    type Error = Error;
    type Fault = IeFault;
    type Presence = Presence;
    type Interfaces = Interfaces;

    #[inline]
    fn ies(octets: &[u8]) -> impl Iterator<Item = Result<((u16, u8), &[u8]), IeFault>> {
        framing::ies(octets)
            .map(|ie| ie.map(|ie| ((ie.ie_type, ie.spare << 4 | ie.instance), ie.value)))
    }

    #[inline]
    fn placeable((ie_type, octet): (u16, u8), content: &[u8]) -> bool {
        !content.is_empty() && octet <= 0x0f && ie_type != 255
    }

    #[inline]
    fn raw(key: (u16, u8), content: &[u8]) -> RawIe {
        ie::raw(key, content)
    }
}

/// Why a message does not decode or encode.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum Error {
    /// Fewer octets than the header that their T flag says (§5.5.1).
    TooShort {
        /// How many there are.
        octets: usize,
    },
    /// A Message Length that is not the length of the octets after the first four
    /// (§7.7.3): a receiver discards the message, and answers a request of a Message
    /// Length shorter than its octets with the Cause "Invalid Length".
    MessageLength {
        /// The header.
        header: Header,
        /// The message type.
        message_type: u8,
        /// The Message Length.
        length: u16,
        /// The octets after the first four.
        available: usize,
    },
    /// IEs whose framing does not fit the message (§7.7.7).
    Ies {
        /// The header.
        header: Header,
        /// The message type.
        message_type: u8,
        /// Where and how.
        fault: IeFault,
    },
    /// The content of an IE longer than its Length can say.
    TooLong {
        /// The IE type.
        ie_type: u16,
        /// The length of the content.
        length: usize,
    },
    /// A message longer than its Message Length can say (§5.5.1).
    MessageTooLong {
        /// Its message type.
        message_type: u8,
        /// Its octets after the first 4, which the Message Length counts.
        length: usize,
    },
    /// A field of the header, or an IE type, whose value its bits cannot hold (§5.5.1,
    /// §8.2.1).
    HeaderField {
        /// The field.
        field: &'static str,
        /// Its value.
        value: u64,
    },
    /// A field longer than its length, its number or its type can say.
    FieldLength {
        /// The IE type.
        ie: &'static str,
        /// The field.
        field: &'static str,
    },
    /// A value that [`Message::encode`] cannot write so that it decodes back to the same
    /// value: a field present without its flag, or missing with it, or missing before a
    /// later field, or octets after a field that runs to the end of the IE.
    Inconsistent {
        /// The IE type.
        ie: &'static str,
        /// The field.
        field: &'static str,
        /// The clause.
        clause: &'static str,
    },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::TooShort { octets } => {
                write!(f, "{octets} octets are fewer than a GTPv2-C header")
            }
            Self::MessageLength {
                length, available, ..
            } => write!(
                f,
                "the Message Length is {length}, and {available} octets follow the first four (TS 29.274 7.7.3)"
            ),
            Self::Ies { fault, .. } => write!(
                f,
                "the IEs do not fit the message: {fault} (TS 29.274 7.7.7)"
            ),
            Self::HeaderField { field, value } => {
                write!(
                    f,
                    "{value} is more than the bits of the {field} hold (TS 29.274 5.5.1)"
                )
            }
            Self::TooLong { ie_type, length } => {
                write!(
                    f,
                    "{length} octets of IE type {ie_type} are more than a Length says"
                )
            }
            Self::MessageTooLong {
                message_type,
                length,
            } => write!(
                f,
                "{length} octets of message type {message_type} are more than a Message Length says (TS 29.274 5.5.1)"
            ),
            Self::FieldLength { ie, field } => {
                write!(f, "{field} of {ie} is longer than its length can say")
            }
            Self::Inconsistent { ie, field, clause } => write!(
                f,
                "{field} of {ie} disagrees with its flags or its other fields (TS 29.274 {clause})"
            ),
        }
    }
}

impl core::error::Error for Error {}

/// The name of an IE in the tree of a scope of `table`: the name of its row, of its type,
/// or its number. An IE of a known type whose name is that of a row of another type goes
/// by its number.
pub(crate) fn tree_name(key: (u16, u8), table: Option<&Table>) -> String {
    if let Some(row) = table.and_then(|table| table.row_of(key)) {
        return row.name.to_string();
    }
    let letters = |name: &str| {
        name.chars()
            .filter(char::is_ascii_alphanumeric)
            .map(|c| c.to_ascii_lowercase())
            .collect::<String>()
    };
    let ie_type = key.0;
    match registry::ie_type_info(ie_type).map(|info| info.name) {
        Some(name)
            if !table.is_some_and(|table| {
                table
                    .rows
                    .iter()
                    .any(|row| letters(row.name) == letters(name))
            }) =>
        {
            name.to_string()
        }
        _ => ie_type.to_string(),
    }
}
