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

//! Framing-only scans: the IEs of a message or of a grouped IE as slices of its octets,
//! with nothing decoded and nothing copied. The codec frames its scopes with the same
//! iterator.
//!
//! ```
//! use oxirush_gtpv2c::framing;
//!
//! // An Echo Request: its Recovery, of instance 0
//! let octets = [0x40, 0x01, 0x00, 0x09, 0x00, 0x00, 0x01, 0x00, 0x03, 0x00, 0x01, 0x00, 0x05];
//! let (header, message_type, ies) = framing::message_ies(&octets)?;
//! assert_eq!((header.sequence, message_type), (1, 1));
//! let recovery = ies.map(Result::unwrap).find(|ie| ie.ie_type == 3).unwrap();
//! assert_eq!((recovery.instance, recovery.value), (0, &[5][..]));
//! # Ok::<(), oxirush_gtpv2c::Error>(())
//! ```

use crate::Error;
use crate::header::Header;
use crate::ie::IeFault;

/// One IE, as a slice of the octets that hold it (§8.2.1).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IeSlice<'a> {
    /// The IE type.
    pub ie_type: u16,
    /// The Length of the IE header: the octets after its first four.
    pub length: u16,
    /// The instance, bits 4 to 1 of octet 4.
    pub instance: u8,
    /// Bits 8 to 5 of octet 4, which are spare.
    pub spare: u8,
    /// The value: the octets after the first four.
    pub value: &'a [u8],
}

impl<'a> IeSlice<'a> {
    /// The IEs of a grouped IE, as slices too.
    pub fn ies(&self) -> Ies<'a> {
        ies(self.value)
    }
}

/// The IEs of a scope, in the order of the wire. After a fault, it ends.
#[derive(Debug, Clone)]
pub struct Ies<'a> {
    rest: &'a [u8],
}

/// The IEs of `octets`: the IEs of a message after its header, or the content of a
/// grouped IE.
pub fn ies(octets: &[u8]) -> Ies<'_> {
    Ies { rest: octets }
}

/// The header, the message type and the IEs of the first message of `octets`, whose
/// Message Length is checked against the octets that it has.
///
/// # Errors
///
/// Octets shorter than a header, and a Message Length that runs past the octets or ends
/// before the header does (§7.7.3).
pub fn message_ies(octets: &[u8]) -> Result<(Header, u8, Ies<'_>), Error> {
    let (header, message_type, length) = Header::decode(octets).ok_or(Error::TooShort {
        octets: octets.len(),
    })?;
    let end = 4 + usize::from(length);
    match octets.get(header.encoded_len()..end) {
        Some(content) => Ok((header, message_type, ies(content))),
        _ => Err(Error::MessageLength {
            header,
            message_type,
            length,
            available: octets.len() - 4,
        }),
    }
}

impl<'a> Iterator for Ies<'a> {
    type Item = Result<IeSlice<'a>, IeFault>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let octets = self.rest;
        if octets.is_empty() {
            return None;
        }
        let Some(&[ie_type, l0, l1, octet4]) = octets.first_chunk::<4>() else {
            self.rest = &[];
            return Some(Err(IeFault::HeaderCut {
                octets: octets.len(),
            }));
        };
        let length = u16::from_be_bytes([l0, l1]);
        let end = 4 + usize::from(length);
        let Some(value) = octets.get(4..end) else {
            self.rest = &[];
            return Some(Err(IeFault::Length {
                ie_type: u16::from(ie_type),
                length,
                available: octets.len() - 4,
            }));
        };
        self.rest = &octets[end..];
        Some(Ok(IeSlice {
            ie_type: u16::from(ie_type),
            length,
            instance: octet4 & 0x0f,
            spare: octet4 >> 4,
            value,
        }))
    }
}
