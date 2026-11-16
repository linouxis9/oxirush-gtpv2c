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

//! Decode, build, check and encode GTPv2-C, the control protocol between the MME, the SGW
//! and the PGW of 4G cores and of N26 between an MME and an AMF (3GPP TS 29.274 V19.6.0):
//! every message and IE of S11, S5/S8, S3, S10, S16 and N26, byte for byte, and their
//! trees as JSON.
//!
//! ```rust
//! use oxirush_gtpv2c::*;
//!
//! fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // An MME asks its SGW whether it is alive: an Echo Request with its Restart Counter
//!     let octets = Message::path(1, EchoRequest::new(Recovery::new(5))).encode()?;
//!     assert_eq!(hex::encode(&octets), "40010009000001000300010005");
//!
//!     // The SGW decodes it and reads its IEs as typed fields
//!     let message = Message::decode(&octets)?;
//!     assert_eq!(message.to_string(), "Echo Request (sequence 1)");
//!     let Body::EchoRequest(echo) = &message.body else { unreachable!() };
//!     assert_eq!(echo.recovery.as_ref().map(|recovery| recovery.value), Some(5));
//!     Ok(())
//! }
//! ```
//!
//! # Tasks
//!
//! The examples of the [README](https://github.com/linouxis9/oxirush-gtpv2c#readme) show
//! these tasks.
//!
//! | Task | Items |
//! |------|-------|
//! | Decode a message and read its IEs | [`Message::decode`], [`Body`], [`Message::path`], [`Message::tunnel`] |
//! | Build a message, and check it | [`CreateSessionRequest::new`], [`FTeid`], [`BearerContextToBeCreatedInCreateSessionRequest`], [`Message::check`], [`Deviation`], [`Cause`] |
//! | Edit a message as JSON | [`Message::to_tree`], [`Message::from_tree`], [`paths`] |
//! | Carry an MM Context on N26 | [`ForwardRelocationRequest`], [`MmContextEpsSecurityContextQuadrupletsAndQuintuplets`] |
//!
//! The facts of the specification are tables of macros: one line per IE type (Table
//! 8.1-1), one row per IE of a table of clause 7, one layout per value. The codec, the
//! check, the trees and the paths are written from them by oxirush-utils.
//!
//! A message decodes and encodes byte-exact: an IE that its table does not have, of an
//! instance that it does not have, with spare bits set, or that does not decode as its
//! type, is kept as its octets in its place.
//!
//! # The OxiRush family
//!
//! - [oxirush-utils](https://docs.rs/oxirush-utils): the 3GPP identities, the JSON tree with its paths, and the fields and tables that the codecs are laid out on.
//! - [oxirush-aper](https://docs.rs/oxirush-aper): aligned PER, the runtime of the NGAP and S1AP codecs.
//! - [oxirush-nas](https://docs.rs/oxirush-nas): the NAS messages of 5GS and EPS.
//! - [oxirush-security](https://docs.rs/oxirush-security): NAS security, AKA and the derivation of keys.
//! - [oxirush-ngap](https://docs.rs/oxirush-ngap): NGAP, between a gNB and an AMF.
//! - [oxirush-s1ap](https://docs.rs/oxirush-s1ap): S1AP, between an eNB and an MME.
//! - [oxirush-gtp-u](https://docs.rs/oxirush-gtp-u): GTP-U tunnels, with a user-plane test peer.
//! - [oxirush-gtpv2c](https://github.com/linouxis9/oxirush-gtpv2c): GTPv2-C, between the MME, the SGW and the PGW, and on N26.
//! - [oxirush-pfcp](https://docs.rs/oxirush-pfcp): PFCP, between the control plane and the user plane.
//! - [oxirush-sctp](https://docs.rs/oxirush-sctp): SCTP sockets, which carry NGAP and S1AP.

#![no_std]
#![deny(unsafe_code)]
#![warn(missing_docs)]
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
mod tft;
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
    AccessRestriction, Key, Later, MmContextEpsSecurityContextQuadrupletsAndQuintuplets, NextHop,
    OldSecurityContext, Quadruplet, Quintuplet,
};
pub use oxirush_utils::fields::Octets;
pub use oxirush_utils::tables::Missing;
pub use registry::*;
pub use table::{Interfaces, On, Presence, Repeat, Role, Row, Table};
pub use tft::{EpsBearerLevelTrafficFlowTemplate, PacketFilter, Tad, TftParameter};
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
    /// A mandatory IE that a message lacks, which its accessor requires (`rat_type()`).
    Missing(oxirush_utils::tables::Missing),
    /// A text that is no value of its field: digits of an IMSI that TBCD does not have, a
    /// name of an APN of an empty label or of a label of more than 63 octets.
    FieldValue {
        /// The IE type.
        ie: &'static str,
        /// The field.
        field: &'static str,
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
            Self::Missing(missing) => write!(f, "{missing}"),
            Self::FieldValue { ie, field } => write!(f, "the text is no {field} of {ie}"),
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

impl From<oxirush_utils::tables::Missing> for Error {
    fn from(missing: oxirush_utils::tables::Missing) -> Self {
        Self::Missing(missing)
    }
}

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
