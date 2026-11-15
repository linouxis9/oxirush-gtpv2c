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

//! The values whose fields their figures make depend on a type or a length, written by
//! hand: Cause with its offending IE (§8.4), PDN Address Allocation (§8.14) and FQ-CSID
//! (§8.62).

use alloc::vec::Vec;
use core::net::{Ipv4Addr, Ipv6Addr};

use crate::Error;
use crate::check::{Deviation, DeviationKind, Path, Rules};
use crate::enums::CauseValue;
use crate::ie::Codec;
use oxirush_utils::fields::Octets;

/// The IE that a Cause names (octets a(n+1) to a(n+4)): its type and its instance, after a
/// Length that is 0.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct OffendingIe {
    /// The IE type.
    pub ie_type: u8,
    /// The Length, which is 0.
    pub length: u16,
    /// Octet a(n+4): the instance, bits 4 to 1, and its spare bits.
    pub instance: u8,
}

/// Cause (§8.4): the Cause value, the flags of the source of an error, and the IE that a
/// rejection concerns.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Cause {
    /// The Cause value (Table 8.4-1).
    pub value: CauseValue,
    /// PCE: the error is of the PDN connection.
    pub pce: bool,
    /// BCE: the error is of the bearer context.
    pub bce: bool,
    /// CS: the error comes from the remote node, not the sender.
    pub cs: bool,
    /// The spare bits 8 to 4 of octet 6.
    pub spare: u8,
    /// The offending IE.
    pub offending_ie: Option<OffendingIe>,
    /// The octets that a release after this one adds.
    pub extra: Octets,
}

impl Cause {
    /// The Cause of `value`.
    pub fn new(value: CauseValue) -> Self {
        Self {
            value,
            ..Self::default()
        }
    }
}

impl Codec for Cause {
    fn decode(octets: &[u8]) -> Option<Self> {
        let [value, flags, rest @ ..] = octets else {
            return None;
        };
        let mut cause = Self {
            value: CauseValue::from_value(*value),
            pce: flags & 0x04 != 0,
            bce: flags & 0x02 != 0,
            cs: flags & 0x01 != 0,
            spare: flags >> 3,
            ..Self::default()
        };
        let rest = match rest {
            [ie_type, l0, l1, instance, after @ ..] => {
                cause.offending_ie = Some(OffendingIe {
                    ie_type: *ie_type,
                    length: u16::from_be_bytes([*l0, *l1]),
                    instance: *instance,
                });
                after
            }
            rest => rest,
        };
        cause.extra = Octets::copied(rest);
        Some(cause)
    }

    fn encode(&self, out: &mut Vec<u8>) -> Result<(), Error> {
        if self.spare > 0x1f {
            return Err(Error::FieldLength {
                ie: "Cause",
                field: "Spare",
            });
        }
        if self.offending_ie.is_none() && !self.extra.is_empty() {
            return Err(Error::Inconsistent {
                ie: "Cause",
                field: "the octets of a later release",
                clause: "8.4",
            });
        }
        out.push(self.value.value());
        out.push(
            self.spare << 3 | u8::from(self.pce) << 2 | u8::from(self.bce) << 1 | u8::from(self.cs),
        );
        if let Some(ie) = &self.offending_ie {
            out.push(ie.ie_type);
            out.extend_from_slice(&ie.length.to_be_bytes());
            out.push(ie.instance);
        }
        out.extend_from_slice(&self.extra);
        Ok(())
    }

    fn encoded_len(&self) -> usize {
        2 + 4 * usize::from(self.offending_ie.is_some()) + self.extra.len()
    }
}

impl Rules for Cause {
    fn check(&self, path: &Path<'_>, found: &mut Vec<Deviation>) {
        oxirush_utils::fields::Field::check(&self.value, path, "8.4", found);
        let spare = self.spare != 0 || self.offending_ie.is_some_and(|ie| ie.instance > 0x0f);
        if spare {
            found.push(Deviation::new(path, "8.4", DeviationKind::SpareBits));
        }
        if self.offending_ie.is_some_and(|ie| ie.length != 0) {
            found.push(Deviation::new(
                path,
                "8.4",
                DeviationKind::Rule("the Length of the offending IE is not 0"),
            ));
        }
    }
}

/// PDN Address Allocation (PAA) (§8.14): the PDN type and the addresses that it has.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Paa {
    /// The PDN Type: 1 IPv4, 2 IPv6, 3 IPv4v6, 4 Non-IP, 5 Ethernet (Table 8.14-1).
    pub pdn_type: u8,
    /// The spare bits 8 to 4 of octet 5.
    pub spare: u8,
    /// The IPv6 prefix length and the IPv6 prefix and interface identifier: of IPv6 and
    /// IPv4v6.
    pub ipv6: Option<(u8, Ipv6Addr)>,
    /// The IPv4 address: of IPv4 and IPv4v6.
    pub ipv4: Option<Ipv4Addr>,
    /// The octets after the addresses of the PDN type, or of another PDN type.
    pub extra: Octets,
}

impl Codec for Paa {
    fn decode(octets: &[u8]) -> Option<Self> {
        let (&first, mut rest) = octets.split_first()?;
        let mut paa = Self {
            pdn_type: first & 0x07,
            spare: first >> 3,
            ..Self::default()
        };
        if matches!(paa.pdn_type, 2 | 3) {
            let (&length, after) = rest.split_first()?;
            let address: [u8; 16] = after.get(..16)?.try_into().ok()?;
            paa.ipv6 = Some((length, Ipv6Addr::from(address)));
            rest = &after[16..];
        }
        if matches!(paa.pdn_type, 1 | 3) {
            let address: [u8; 4] = rest.get(..4)?.try_into().ok()?;
            paa.ipv4 = Some(Ipv4Addr::from(address));
            rest = &rest[4..];
        }
        paa.extra = Octets::copied(rest);
        Some(paa)
    }

    fn encode(&self, out: &mut Vec<u8>) -> Result<(), Error> {
        let (v6, v4) = (
            matches!(self.pdn_type, 2 | 3),
            matches!(self.pdn_type, 1 | 3),
        );
        if self.pdn_type > 7 || self.spare > 0x1f {
            return Err(Error::FieldLength {
                ie: "PDN Address Allocation (PAA)",
                field: "PDN Type",
            });
        }
        if self.ipv6.is_some() != v6 || self.ipv4.is_some() != v4 {
            return Err(Error::Inconsistent {
                ie: "PDN Address Allocation (PAA)",
                field: "PDN Address and Prefix",
                clause: "8.14",
            });
        }
        out.push(self.spare << 3 | self.pdn_type);
        if let Some((length, address)) = &self.ipv6 {
            out.push(*length);
            out.extend_from_slice(&address.octets());
        }
        if let Some(address) = &self.ipv4 {
            out.extend_from_slice(&address.octets());
        }
        out.extend_from_slice(&self.extra);
        Ok(())
    }

    fn encoded_len(&self) -> usize {
        1 + self.ipv6.map_or(0, |_| 17) + self.ipv4.map_or(0, |_| 4) + self.extra.len()
    }
}

impl Rules for Paa {
    fn check(&self, path: &Path<'_>, found: &mut Vec<Deviation>) {
        if self.spare != 0 {
            found.push(Deviation::new(path, "8.14", DeviationKind::SpareBits));
        }
        if !(1..=5).contains(&self.pdn_type) {
            found.push(Deviation::new(
                path,
                "8.14",
                DeviationKind::ReservedValue {
                    value: self.pdn_type.into(),
                },
            ));
        }
    }
}

/// The Node-ID of an FQ-CSID, by its Node-ID Type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeId {
    /// 0: an IPv4 address.
    Ipv4(Ipv4Addr),
    /// 1: an IPv6 address.
    Ipv6(Ipv6Addr),
    /// 2: the MCC and the MNC (20 bits, MCC × 1000 + MNC) and a number of 12 bits.
    Plmn(u32),
}

impl Default for NodeId {
    fn default() -> Self {
        Self::Ipv4(Ipv4Addr::UNSPECIFIED)
    }
}

/// Fully qualified PDN Connection Set Identifier (FQ-CSID) (§8.62): a node and its CSIDs.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct FqCsid {
    /// The node.
    pub node_id: NodeId,
    /// The CSIDs, at most 15.
    pub csids: Vec<u16>,
    /// The octets that a release after this one adds.
    pub extra: Octets,
}

impl Codec for FqCsid {
    fn decode(octets: &[u8]) -> Option<Self> {
        let (&first, rest) = octets.split_first()?;
        let (kind, count) = (first >> 4, usize::from(first & 0x0f));
        let (node_id, rest) = match kind {
            0 => (
                NodeId::Ipv4(Ipv4Addr::from(<[u8; 4]>::try_from(rest.get(..4)?).ok()?)),
                &rest[4..],
            ),
            1 => (
                NodeId::Ipv6(Ipv6Addr::from(<[u8; 16]>::try_from(rest.get(..16)?).ok()?)),
                &rest[16..],
            ),
            2 => (
                NodeId::Plmn(u32::from_be_bytes(rest.get(..4)?.try_into().ok()?)),
                &rest[4..],
            ),
            _ => return None,
        };
        let csids = rest.get(..2 * count)?;
        Some(Self {
            node_id,
            csids: csids
                .chunks(2)
                .map(|pair| u16::from_be_bytes([pair[0], pair[1]]))
                .collect(),
            extra: Octets::copied(&rest[2 * count..]),
        })
    }

    fn encode(&self, out: &mut Vec<u8>) -> Result<(), Error> {
        let count = u8::try_from(self.csids.len())
            .ok()
            .filter(|count| *count < 16);
        let count = count.ok_or(Error::FieldLength {
            ie: "FQ-CSID",
            field: "Number of CSIDs",
        })?;
        let kind = match self.node_id {
            NodeId::Ipv4(_) => 0,
            NodeId::Ipv6(_) => 1,
            NodeId::Plmn(_) => 2,
        };
        out.push(kind << 4 | count);
        match self.node_id {
            NodeId::Ipv4(address) => out.extend_from_slice(&address.octets()),
            NodeId::Ipv6(address) => out.extend_from_slice(&address.octets()),
            NodeId::Plmn(number) => out.extend_from_slice(&number.to_be_bytes()),
        }
        for csid in &self.csids {
            out.extend_from_slice(&csid.to_be_bytes());
        }
        out.extend_from_slice(&self.extra);
        Ok(())
    }

    fn encoded_len(&self) -> usize {
        let node = match self.node_id {
            NodeId::Ipv6(_) => 16,
            _ => 4,
        };
        1 + node + 2 * self.csids.len() + self.extra.len()
    }
}

impl Rules for FqCsid {}

macro_rules! consts {
    ($($ty:ident [$($name:literal),*];)*) => {$(
        impl $ty {
            /// The names of its fields, as its tree writes them.
            pub(crate) const FIELDS: &'static [&'static str] = &[$($name),*];
            /// The members of its tree.
            pub(crate) const MEMBERS: &'static [&'static str] = Self::FIELDS;
            /// What its tree takes.
            #[cfg(feature = "tree")]
            pub(crate) const TAKES: &'static oxirush_utils::tree::Takes = &oxirush_utils::tree::Takes::Members(None, &[]);
            /// Its layout, which the test spec does not compare yet.
            #[doc(hidden)]
            #[cfg(feature = "spec")]
            pub const LAYOUT: Option<&'static oxirush_utils::fields::Layout> = None;
        }
    )*};
}

consts! {
    Cause ["Cause value", "PCE", "BCE", "CS", "Spare", "Offending IE", "unknown octets"];
    Paa ["PDN Type", "Spare", "IPv6 Prefix Length", "IPv6 address", "IPv4 address", "unknown octets"];
    FqCsid ["Node-ID", "PDN Connection Set Identifier (CSID)", "unknown octets"];
}

#[cfg(feature = "tree")]
mod tree_form {
    use oxirush_utils::fields::FieldTree;
    use oxirush_utils::tree::{self, Members, Reading, Refused, Tree, Value};

    use super::*;

    fn flag(m: &mut Members, name: &'static str, set: bool) {
        if set {
            m.put(name, Value::Bool(true));
        }
    }

    fn read_flag(
        read: &tree::Read<'_>,
        name: &'static str,
        reading: &mut Reading,
    ) -> Result<bool, Refused> {
        Ok(reading
            .optional(read, name, |v, r| r.boolean(v))?
            .unwrap_or(false))
    }

    fn extra(read: &tree::Read<'_>, reading: &mut Reading) -> Result<Octets, Refused> {
        Ok(reading
            .optional(read, "unknown octets", |v, r| r.octets(v))?
            .unwrap_or_default()
            .into())
    }

    /// `{"Cause value": "Mandatory IE missing", "Offending IE": {"Type": 82, "Instance": 0}}`.
    impl Tree for Cause {
        const NAME: &'static str = "Cause";
        fn to_tree(&self) -> Value {
            let mut m = Members::new();
            m.put("Cause value", self.value.to_tree());
            flag(&mut m, "PCE", self.pce);
            flag(&mut m, "BCE", self.bce);
            flag(&mut m, "CS", self.cs);
            if self.spare != 0 {
                m.put("Spare", tree::unsigned(self.spare.into()));
            }
            if let Some(ie) = &self.offending_ie {
                let mut e = Members::new();
                e.put("Type", tree::unsigned(ie.ie_type.into()));
                if ie.length != 0 {
                    e.put("Length", tree::unsigned(ie.length.into()));
                }
                e.put("Instance", tree::unsigned((ie.instance & 0x0f).into()));
                if ie.instance > 0x0f {
                    e.put("Spare", tree::unsigned((ie.instance >> 4).into()));
                }
                m.put("Offending IE", e.finish());
            }
            if !self.extra.is_empty() {
                m.put("unknown octets", tree::octets(&self.extra));
            }
            m.finish()
        }
        fn from_tree(value: &Value, reading: &mut Reading) -> Result<Self, Refused> {
            let read = reading.members(value, Self::FIELDS)?;
            Ok(Self {
                value: reading.member(&read, "Cause value", CauseValue::from_tree)?,
                pce: read_flag(&read, "PCE", reading)?,
                bce: read_flag(&read, "BCE", reading)?,
                cs: read_flag(&read, "CS", reading)?,
                spare: reading
                    .optional(&read, "Spare", |v, r| r.unsigned(v, 0, 0x1f))?
                    .unwrap_or(0) as u8,
                offending_ie: reading.optional(&read, "Offending IE", |v, r| {
                    let e = r.members(v, &["Type", "Length", "Instance", "Spare"])?;
                    let instance = r
                        .optional(&e, "Instance", |v, r| r.unsigned(v, 0, 15))?
                        .unwrap_or(0) as u8;
                    let spare = r
                        .optional(&e, "Spare", |v, r| r.unsigned(v, 0, 15))?
                        .unwrap_or(0) as u8;
                    Ok(OffendingIe {
                        ie_type: r.member(&e, "Type", |v, r| r.unsigned(v, 0, 255))? as u8,
                        length: r
                            .optional(&e, "Length", |v, r| r.unsigned(v, 0, 65535))?
                            .unwrap_or(0) as u16,
                        instance: spare << 4 | instance,
                    })
                })?,
                extra: extra(&read, reading)?,
            })
        }
    }

    fn address<T: core::str::FromStr>(
        v: &Value,
        r: &mut Reading,
        what: &str,
    ) -> Result<T, Refused> {
        v.as_str()
            .and_then(|text| text.parse().ok())
            .ok_or_else(|| r.mismatch(what, v))
    }

    /// `{"PDN Type": 3, "IPv6 Prefix Length": 64, "IPv6 address": "2001:db8::1", "IPv4 address": "10.45.0.2"}`.
    impl Tree for Paa {
        const NAME: &'static str = "PDN Address Allocation (PAA)";
        fn to_tree(&self) -> Value {
            let mut m = Members::new();
            m.put("PDN Type", tree::unsigned(self.pdn_type.into()));
            if self.spare != 0 {
                m.put("Spare", tree::unsigned(self.spare.into()));
            }
            if let Some((length, address)) = &self.ipv6 {
                m.put("IPv6 Prefix Length", tree::unsigned((*length).into()));
                m.put("IPv6 address", tree::text(alloc::format!("{address}")));
            }
            if let Some(address) = &self.ipv4 {
                m.put("IPv4 address", tree::text(alloc::format!("{address}")));
            }
            if !self.extra.is_empty() {
                m.put("unknown octets", tree::octets(&self.extra));
            }
            m.finish()
        }
        fn from_tree(value: &Value, reading: &mut Reading) -> Result<Self, Refused> {
            let read = reading.members(value, Self::FIELDS)?;
            let pdn_type = reading.member(&read, "PDN Type", |v, r| r.unsigned(v, 0, 7))? as u8;
            let length =
                reading.optional(&read, "IPv6 Prefix Length", |v, r| r.unsigned(v, 0, 255))?;
            let ipv6 = reading.optional(&read, "IPv6 address", |v, r| {
                address::<Ipv6Addr>(v, r, "an IPv6 address")
            })?;
            Ok(Self {
                pdn_type,
                spare: reading
                    .optional(&read, "Spare", |v, r| r.unsigned(v, 0, 0x1f))?
                    .unwrap_or(0) as u8,
                ipv6: ipv6.map(|address| (length.unwrap_or(64) as u8, address)),
                ipv4: reading.optional(&read, "IPv4 address", |v, r| {
                    address::<Ipv4Addr>(v, r, "an IPv4 address")
                })?,
                extra: extra(&read, reading)?,
            })
        }
    }

    /// `{"Node-ID": "10.0.0.1", "PDN Connection Set Identifier (CSID)": [1, 2]}`, a Node-ID
    /// of type 2 as `{"MCC MNC and number": n}`.
    impl Tree for FqCsid {
        const NAME: &'static str = "FQ-CSID";
        fn to_tree(&self) -> Value {
            let mut m = Members::new();
            let node = match self.node_id {
                NodeId::Ipv4(address) => tree::text(alloc::format!("{address}")),
                NodeId::Ipv6(address) => tree::text(alloc::format!("{address}")),
                NodeId::Plmn(number) => {
                    tree::alternative("MCC MNC and number", tree::unsigned(number.into()))
                }
            };
            m.put("Node-ID", node);
            let csids = self.csids.iter().map(|csid| tree::unsigned((*csid).into()));
            m.put(
                "PDN Connection Set Identifier (CSID)",
                Value::Array(csids.collect()),
            );
            if !self.extra.is_empty() {
                m.put("unknown octets", tree::octets(&self.extra));
            }
            m.finish()
        }
        fn from_tree(value: &Value, reading: &mut Reading) -> Result<Self, Refused> {
            let read = reading.members(value, Self::FIELDS)?;
            let node_id = reading.member(&read, "Node-ID", |v, r| {
                if let Some(text) = v.as_str() {
                    if let Ok(address) = text.parse::<Ipv4Addr>() {
                        return Ok(NodeId::Ipv4(address));
                    }
                    if let Ok(address) = text.parse::<Ipv6Addr>() {
                        return Ok(NodeId::Ipv6(address));
                    }
                    return Err(r.mismatch("an address, or {\"MCC MNC and number\": n}", v));
                }
                let (_, content) = r.alternative(v, &["MCC MNC and number"])?;
                r.within("MCC MNC and number", content, |v, r| {
                    r.unsigned(v, 0, u32::MAX.into())
                })
                .map(|n| NodeId::Plmn(n as u32))
            })?;
            let csids = reading.member(&read, "PDN Connection Set Identifier (CSID)", |v, r| {
                r.list(v, |e, r| r.unsigned(e, 0, 65535).map(|csid| csid as u16))
            })?;
            Ok(Self {
                node_id,
                csids,
                extra: extra(&read, reading)?,
            })
        }
    }
}
