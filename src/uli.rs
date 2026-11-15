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

//! User Location Information (§8.21, IE type 86): the identities of the place of the UE,
//! each after its PLMN. A flag of octet 5 is the presence of its identity.

use alloc::vec::Vec;

use crate::Error;
use crate::check::{Deviation, DeviationKind, Path, Rules};
use crate::ie::Codec;
use oxirush_utils::fields::{Octets, PlmnId};

/// An identity of a ULI: its PLMN, and its codes of the octets after it, as its figure
/// (Figures 8.21-2 to 8.21-9) has them: an LAC and a CI, a TAC, an ECI of 28 bits.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Location {
    /// The PLMN.
    pub plmn: PlmnId,
    /// Its first code: the LAC (CGI, SAI, RAI, LAI), the TAC (TAI), the ECI (ECGI), or
    /// the eNodeB ID (Macro and Extended Macro eNodeB ID).
    pub code: u32,
    /// Its second code: the CI (CGI), the SAC (SAI), the RAC (RAI); 0 for the others.
    pub second: u16,
    /// The bits of the identity that its figure has spare, or SMeNB (bit 8 of the first
    /// octet after the PLMN of an Extended Macro eNodeB ID), as written.
    pub spare: u8,
}

/// The kinds of identities of a ULI, in the order of their flags from bit 1: the octets
/// after the PLMN, the names of their codes, and the bits of the first code.
const KINDS: [(&str, usize, &str, Option<&str>, u32); 8] = [
    ("CGI", 4, "LAC", Some("CI"), 16),
    ("SAI", 4, "LAC", Some("SAC"), 16),
    ("RAI", 4, "LAC", Some("RAC"), 16),
    ("TAI", 2, "TAC", None, 16),
    ("ECGI", 4, "ECI", None, 28),
    ("LAI", 2, "LAC", None, 16),
    ("Macro eNodeB ID", 3, "Macro eNodeB ID", None, 20),
    (
        "Extended Macro eNodeB ID",
        3,
        "Extended Macro eNodeB ID",
        None,
        21,
    ),
];

/// User Location Information (ULI) (§8.21): the identities that are there, by their flag.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Uli {
    /// The identities by their flag, bit 1 (CGI) to bit 8 (Extended Macro eNodeB ID).
    pub locations: [Option<Location>; 8],
    /// The octets that a release after this one adds.
    pub extra: Octets,
}

impl Uli {
    /// The TAI, the ECGI: the identity of the kind of `name`, `"TAI"`.
    pub fn get(&self, name: &str) -> Option<&Location> {
        let at = KINDS.iter().position(|kind| kind.0 == name)?;
        self.locations[at].as_ref()
    }
}

impl Codec for Uli {
    fn decode(octets: &[u8]) -> Option<Self> {
        let (&flags, mut rest) = octets.split_first()?;
        let mut value = Self::default();
        for (at, (_, size, _, second, bits)) in KINDS.iter().enumerate() {
            if flags >> at & 1 == 0 {
                continue;
            }
            let (identity, after) = rest.split_at_checked(3 + size)?;
            rest = after;
            let plmn = PlmnId([identity[0], identity[1], identity[2]]);
            let codes = &identity[3..];
            let mut location = Location {
                plmn,
                ..Location::default()
            };
            let first = match second {
                Some(_) => {
                    location.second = u16::from_be_bytes([codes[2], codes[3]]);
                    &codes[..2]
                }
                None => codes,
            };
            let number = first
                .iter()
                .fold(0u32, |n, octet| n << 8 | u32::from(*octet));
            location.code = number & ((1u32 << bits) - 1);
            location.spare = (number >> bits) as u8;
            value.locations[at] = Some(location);
        }
        value.extra = Octets::copied(rest);
        Some(value)
    }

    fn encode(&self, out: &mut Vec<u8>) -> Result<(), Error> {
        let flags = self
            .locations
            .iter()
            .enumerate()
            .fold(0u8, |flags, (at, l)| flags | u8::from(l.is_some()) << at);
        out.push(flags);
        for (location, (name, size, _, second, bits)) in self.locations.iter().zip(KINDS) {
            let Some(location) = location else { continue };
            let width = if second.is_some() {
                16
            } else {
                8 * size as u32
            };
            let spare_bits = width - bits;
            if location.code >> bits != 0 || (spare_bits < 8 && location.spare >> spare_bits != 0) {
                return Err(Error::FieldLength {
                    ie: "User Location Information (ULI)",
                    field: name,
                });
            }
            out.extend_from_slice(&location.plmn.0);
            let number = u32::from(location.spare) << bits | location.code;
            let octets = width as usize / 8;
            out.extend_from_slice(&number.to_be_bytes()[4 - octets..]);
            if second.is_some() {
                out.extend_from_slice(&location.second.to_be_bytes());
            }
        }
        out.extend_from_slice(&self.extra);
        Ok(())
    }

    fn encoded_len(&self) -> usize {
        let identities = self
            .locations
            .iter()
            .zip(KINDS)
            .filter(|(l, _)| l.is_some());
        1 + identities.map(|(_, kind)| 3 + kind.1).sum::<usize>() + self.extra.len()
    }
}

impl Rules for Uli {
    fn check(&self, path: &Path<'_>, found: &mut Vec<Deviation>) {
        // The spare bits, but SMeNB of an Extended Macro eNodeB ID (bit 8 of its octet)
        let spare = |at: usize, location: &Location| match at {
            7 => location.spare & 0x03 != 0,
            _ => location.spare != 0,
        };
        let set = self
            .locations
            .iter()
            .enumerate()
            .any(|(at, l)| l.as_ref().is_some_and(|l| spare(at, l)));
        if set {
            found.push(Deviation::new(path, "8.21", DeviationKind::SpareBits));
        }
    }
}

impl Uli {
    /// The names of its fields, as its tree writes them.
    pub(crate) const FIELDS: &'static [&'static str] = &[
        "CGI",
        "SAI",
        "RAI",
        "TAI",
        "ECGI",
        "LAI",
        "Macro eNodeB ID",
        "Extended Macro eNodeB ID",
        "unknown octets",
    ];
    /// The members of its tree.
    pub(crate) const MEMBERS: &'static [&'static str] = Self::FIELDS;
    /// What its tree takes.
    #[cfg(feature = "tree")]
    pub(crate) const TAKES: &'static oxirush_utils::tree::Takes =
        &oxirush_utils::tree::Takes::Members(None, &[]);
    /// Its layout, which the test spec does not compare yet.
    #[doc(hidden)]
    #[cfg(feature = "spec")]
    pub const LAYOUT: Option<&'static oxirush_utils::fields::Layout> = None;
}

/// `{"TAI": {"PLMN": "208-93", "TAC": 1}, "ECGI": {"PLMN": "208-93", "ECI": 257}}`: each
/// identity that is there, its spare bits where they are set, and SMeNB of an Extended
/// Macro eNodeB ID.
#[cfg(feature = "tree")]
mod tree_form {
    use oxirush_utils::fields::FieldTree;
    use oxirush_utils::tree::{self, Members, Reading, Refused, Tree, Value};

    use super::*;

    const MEMBERS: [&[&str]; 8] = [
        &["PLMN", "LAC", "CI", "Spare"],
        &["PLMN", "LAC", "SAC", "Spare"],
        &["PLMN", "LAC", "RAC", "Spare"],
        &["PLMN", "TAC", "Spare"],
        &["PLMN", "ECI", "Spare"],
        &["PLMN", "LAC", "Spare"],
        &["PLMN", "Macro eNodeB ID", "Spare"],
        &["PLMN", "SMeNB", "Extended Macro eNodeB ID", "Spare"],
    ];

    impl Tree for Uli {
        const NAME: &'static str = "User Location Information (ULI)";

        fn to_tree(&self) -> Value {
            let mut m = Members::new();
            for (at, (location, (name, _, code, second, _))) in
                self.locations.iter().zip(KINDS).enumerate()
            {
                let Some(location) = location else { continue };
                let mut e = Members::new();
                e.put("PLMN", location.plmn.to_tree());
                if at == 7 && location.spare & 0x04 != 0 {
                    e.put("SMeNB", Value::Bool(true));
                }
                e.put(code, tree::unsigned(location.code.into()));
                if let Some(second) = second {
                    e.put(second, tree::unsigned(location.second.into()));
                }
                let spare = if at == 7 {
                    location.spare & 0x03
                } else {
                    location.spare
                };
                if spare != 0 {
                    e.put("Spare", tree::unsigned(spare.into()));
                }
                m.put(name, e.finish());
            }
            if !self.extra.is_empty() {
                m.put("unknown octets", tree::octets(&self.extra));
            }
            m.finish()
        }

        fn from_tree(value: &Value, reading: &mut Reading) -> Result<Self, Refused> {
            let read = reading.members(value, Self::FIELDS)?;
            let mut uli = Self::default();
            for (at, (name, _, code, second, bits)) in KINDS.into_iter().enumerate() {
                uli.locations[at] = reading.optional(&read, name, |v, r| {
                    let e = r.members(v, MEMBERS[at])?;
                    let plmn = r.member(&e, "PLMN", PlmnId::from_tree)?;
                    let code =
                        r.member(&e, code, |v, r| r.unsigned(v, 0, (1u64 << bits) - 1))? as u32;
                    let second = match second {
                        Some(name) => r.member(&e, name, |v, r| r.unsigned(v, 0, 0xffff))? as u16,
                        None => 0,
                    };
                    let mut spare = r
                        .optional(&e, "Spare", |v, r| r.unsigned(v, 0, 0xff))?
                        .unwrap_or(0) as u8;
                    if at == 7
                        && r.optional(&e, "SMeNB", |v, r| r.boolean(v))?
                            .unwrap_or(false)
                    {
                        spare |= 0x04;
                    }
                    Ok(Location {
                        plmn,
                        code,
                        second,
                        spare,
                    })
                })?;
            }
            uli.extra = reading
                .optional(&read, "unknown octets", |v, r| r.octets(v))?
                .unwrap_or_default()
                .into();
            Ok(uli)
        }
    }
}
