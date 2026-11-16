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

//! MM Context of an EPS security context and quadruplets (§8.38, Figure 8.38-5, IE type
//! 107): the context that an MME gives an MME or an AMF over S10 and N26. Its keys are
//! octets that `Debug` does not print, and its tree is read as a secret.

use alloc::vec::Vec;
use core::fmt;

use crate::Error;
use crate::check::{Deviation, DeviationKind, Path, Rules};
use crate::enums::{NasCipher, NasIntegrity, SecurityMode};
use crate::ie::Codec;
use oxirush_utils::fields::Octets;

/// The octets of a key, which `Debug` does not print: their number alone.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Key<const N: usize>(pub [u8; N]);

impl<const N: usize> Default for Key<N> {
    fn default() -> Self {
        Self([0; N])
    }
}

impl<const N: usize> fmt::Debug for Key<N> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Key({N} octets)")
    }
}

impl<const N: usize> From<[u8; N]> for Key<N> {
    fn from(octets: [u8; N]) -> Self {
        Self(octets)
    }
}

/// The Next Hop of an EPS security context and its chaining count (NHI): NH and the octet
/// of NCC.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct NextHop {
    /// NH.
    pub nh: Key<32>,
    /// NCC, of 3 bits.
    pub ncc: u8,
    /// The spare bits 8 to 4 of the octet of NCC.
    pub spare: u8,
}

/// An authentication quadruplet (Figure 8.38-9).
#[derive(Clone, Default, PartialEq, Eq)]
pub struct Quadruplet {
    /// RAND.
    pub rand: [u8; 16],
    /// XRES, after its length.
    pub xres: Octets,
    /// AUTN, after its length.
    pub autn: Octets,
    /// KASME.
    pub kasme: Key<32>,
}

/// An authentication quintuplet (Figure 8.38-8).
#[derive(Clone, Default, PartialEq, Eq)]
pub struct Quintuplet {
    /// RAND.
    pub rand: [u8; 16],
    /// XRES, after its length.
    pub xres: Octets,
    /// CK.
    pub ck: Key<16>,
    /// IK.
    pub ik: Key<16>,
    /// AUTN, after its length.
    pub autn: Octets,
}

/// The old EPS security context, which an S10 Forward Relocation Request may have (OSCI,
/// octets s to s+64).
#[derive(Clone, Default, PartialEq, Eq)]
pub struct OldSecurityContext {
    /// RLOS: the UE is attached for restricted local operator services.
    pub rlos: bool,
    /// The old KSIASME, of 3 bits.
    pub ksi_asme: u8,
    /// The old NCC, of 3 bits, which a receiver ignores without the old NH.
    pub ncc: u8,
    /// The old KASME.
    pub kasme: Key<32>,
    /// The old NH (NHI_old).
    pub nh: Option<Key<32>>,
}

/// The access restriction data of octet r+1: each a RAT that the UE is not allowed.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct AccessRestriction {
    /// ECNA: Enhanced Coverage Not Allowed.
    pub ecna: bool,
    /// NBNA: NB-IoT Not Allowed.
    pub nbna: bool,
    /// HNNA: HO-To-Non-3GPP-Access Not Allowed.
    pub hnna: bool,
    /// ENA: WB-E-UTRAN Not Allowed.
    pub ena: bool,
    /// INA: I-HSPA-Evolution Not Allowed.
    pub ina: bool,
    /// GANA: GAN Not Allowed.
    pub gana: bool,
    /// GENA: GERAN Not Allowed.
    pub gena: bool,
    /// UNA: UTRAN Not Allowed.
    pub una: bool,
}

impl AccessRestriction {
    #[cfg_attr(not(feature = "tree"), allow(dead_code))]
    const NAMES: [&'static str; 8] = ["ECNA", "NBNA", "HNNA", "ENA", "INA", "GANA", "GENA", "UNA"];

    fn bits(self) -> [bool; 8] {
        [
            self.ecna, self.nbna, self.hnna, self.ena, self.ina, self.gana, self.gena, self.una,
        ]
    }

    fn of(octet: u8) -> Self {
        let bit = |n: u8| octet >> n & 1 == 1;
        Self {
            ecna: bit(7),
            nbna: bit(6),
            hnna: bit(5),
            ena: bit(4),
            ina: bit(3),
            gana: bit(2),
            gena: bit(1),
            una: bit(0),
        }
    }

    fn octet(self) -> u8 {
        self.bits()
            .iter()
            .fold(0, |octet, bit| octet << 1 | u8::from(*bit))
    }
}

/// The fields after the access restriction data (octets w to a), which a sender of an
/// earlier release leaves out: each is there when the octets go on to it.
#[derive(Clone, Default, PartialEq, Eq)]
pub struct Later {
    /// Voice Domain Preference and UE's Usage Setting, after its length (TS 24.008
    /// clause 10.5.5.28).
    pub voice_domain_preference: Option<Octets>,
    /// UE Radio Capability for Paging information, after its length of two octets.
    pub ue_radio_capability_for_paging: Option<Octets>,
    /// Extended Access Restriction Data, after its length.
    pub extended_access_restriction: Option<Octets>,
    /// UE additional security capability, after its length (TS 24.301 clause 9.9.3.53).
    pub ue_additional_security_capability: Option<Octets>,
    /// UE NR security capability, after its length (TS 24.501 clause 9.11.3.54).
    pub ue_nr_security_capability: Option<Octets>,
    /// APN Rate Control Statuses, after their length of two octets.
    pub apn_rate_control_statuses: Option<Octets>,
    /// Core Network Restrictions, after its length.
    pub core_network_restrictions: Option<Octets>,
    /// UE Radio Capability ID, after its length.
    pub ue_radio_capability_id: Option<Octets>,
    /// Octet a: TRIDI (bit 3), ENSCT (bits 2 and 1) and its spare bits.
    pub octet_a: Option<u8>,
}

/// MM Context (EPS Security Context, Quadruplets and Quintuplets) (§8.38, Figure 8.38-5).
/// A flag of the figure is the presence of what it announces: DRXI of `drx_parameter`,
/// NHI of `nh`, SAMBRI of `subscribed_ue_ambr`, UAMBRI of `used_ue_ambr`, OSCI of
/// `old_security_context`.
#[derive(Clone, Default, PartialEq, Eq)]
pub struct MmContextEpsSecurityContextQuadrupletsAndQuintuplets {
    /// Security Mode, of Table 8.38-1: 4 for this context.
    pub security_mode: SecurityMode,
    /// KSIASME, of 3 bits.
    pub ksi_asme: u8,
    /// Used NAS integrity protection algorithm, of Table 8.38-4.
    pub used_nas_integrity: NasIntegrity,
    /// Used NAS Cipher, of Table 8.38-2.
    pub used_nas_cipher: NasCipher,
    /// NAS Downlink Count, of 24 bits.
    pub nas_downlink_count: u32,
    /// NAS Uplink Count, of 24 bits.
    pub nas_uplink_count: u32,
    /// KASME.
    pub kasme: Key<32>,
    /// The authentication quadruplets, at most 7 by the 3 bits of their number.
    pub quadruplets: Vec<Quadruplet>,
    /// The authentication quintuplets, at most 7.
    pub quintuplets: Vec<Quintuplet>,
    /// DRX parameter (TS 24.008 clause 10.5.5.6): DRXI.
    pub drx_parameter: Option<[u8; 2]>,
    /// NH and NCC: NHI.
    pub nh: Option<NextHop>,
    /// Uplink and downlink Subscribed UE AMBR, in kbps: SAMBRI.
    pub subscribed_ue_ambr: Option<(u32, u32)>,
    /// Uplink and downlink Used UE AMBR, in kbps: UAMBRI.
    pub used_ue_ambr: Option<(u32, u32)>,
    /// UE Network Capability, after its length (TS 24.301 clause 9.9.3.34).
    pub ue_network_capability: Octets,
    /// MS Network Capability, after its length (TS 24.008 clause 10.5.5.12).
    pub ms_network_capability: Octets,
    /// Mobile Equipment Identity, after its length, as §8.10 writes it.
    pub mei: Octets,
    /// The access restriction data of octet r+1.
    pub access_restriction: AccessRestriction,
    /// The old EPS security context: OSCI.
    pub old_security_context: Option<OldSecurityContext>,
    /// The fields of later releases.
    pub later: Later,
    /// The octets that a release after this one adds.
    pub extra: Octets,
}

impl fmt::Debug for MmContextEpsSecurityContextQuadrupletsAndQuintuplets {
    /// The context without its keys and vectors: their number alone.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MmContextEpsSecurityContextQuadrupletsAndQuintuplets")
            .field("security_mode", &self.security_mode)
            .field("ksi_asme", &self.ksi_asme)
            .field("used_nas_integrity", &self.used_nas_integrity)
            .field("used_nas_cipher", &self.used_nas_cipher)
            .field("nas_downlink_count", &self.nas_downlink_count)
            .field("nas_uplink_count", &self.nas_uplink_count)
            .field("quadruplets", &self.quadruplets.len())
            .field("quintuplets", &self.quintuplets.len())
            .field("nh", &self.nh.is_some())
            .field("old_security_context", &self.old_security_context.is_some())
            .finish_non_exhaustive()
    }
}

/// The octets of a value, read in order.
struct Reader<'a>(&'a [u8]);

impl<'a> Reader<'a> {
    fn take(&mut self, count: usize) -> Option<&'a [u8]> {
        let (taken, rest) = self.0.split_at_checked(count)?;
        self.0 = rest;
        Some(taken)
    }
    fn octet(&mut self) -> Option<u8> {
        self.take(1).map(|octets| octets[0])
    }
    fn array<const N: usize>(&mut self) -> Option<[u8; N]> {
        self.take(N)?.try_into().ok()
    }
    fn u24(&mut self) -> Option<u32> {
        self.take(3)
            .map(|o| u32::from_be_bytes([0, o[0], o[1], o[2]]))
    }
    fn u32(&mut self) -> Option<u32> {
        self.array::<4>().map(u32::from_be_bytes)
    }
    fn lv8(&mut self) -> Option<Octets> {
        let length = self.octet()?;
        self.take(usize::from(length)).map(Octets::copied)
    }
    fn lv16(&mut self) -> Option<Octets> {
        let length = u16::from_be_bytes(self.array::<2>()?);
        self.take(usize::from(length)).map(Octets::copied)
    }
    /// A field of a later release: none where the octets end before it.
    fn later<T>(&mut self, read: impl FnOnce(&mut Self) -> Option<T>) -> Option<Option<T>> {
        match self.0.is_empty() {
            true => Some(None),
            false => read(self).map(Some),
        }
    }
}

fn lv8(octets: &Octets, out: &mut Vec<u8>) {
    out.push(u8::try_from(octets.len()).unwrap_or(u8::MAX));
    out.extend_from_slice(octets);
}

fn lv16(octets: &Octets, out: &mut Vec<u8>) {
    out.extend_from_slice(
        &u16::try_from(octets.len())
            .unwrap_or(u16::MAX)
            .to_be_bytes(),
    );
    out.extend_from_slice(octets);
}

const IE: &str = "MM Context (EPS Security Context, Quadruplets and Quintuplets)";

impl Codec for MmContextEpsSecurityContextQuadrupletsAndQuintuplets {
    fn decode(octets: &[u8]) -> Option<Self> {
        let mut r = Reader(octets);
        let [o5, o6, o7] = r.array::<3>()?;
        let (nhi, drxi) = (o5 & 0x10 != 0, o5 & 0x08 != 0);
        let (quintuplets, quadruplets) = (o6 >> 5, o6 >> 2 & 0x07);
        let (uambri, osci, sambri) = (o6 & 0x02 != 0, o6 & 0x01 != 0, o7 & 0x80 != 0);
        let mut value = Self {
            security_mode: SecurityMode::from_value(o5 >> 5),
            ksi_asme: o5 & 0x07,
            used_nas_integrity: NasIntegrity::from_value(o7 >> 4 & 0x07),
            used_nas_cipher: NasCipher::from_value(o7 & 0x0f),
            nas_downlink_count: r.u24()?,
            nas_uplink_count: r.u24()?,
            kasme: Key(r.array()?),
            ..Self::default()
        };
        for _ in 0..quadruplets {
            value.quadruplets.push(Quadruplet {
                rand: r.array()?,
                xres: r.lv8()?,
                autn: r.lv8()?,
                kasme: Key(r.array()?),
            });
        }
        for _ in 0..quintuplets {
            value.quintuplets.push(Quintuplet {
                rand: r.array()?,
                xres: r.lv8()?,
                ck: Key(r.array()?),
                ik: Key(r.array()?),
                autn: r.lv8()?,
            });
        }
        value.drx_parameter = if drxi { Some(r.array()?) } else { None };
        value.nh = if nhi {
            let (nh, octet) = (Key(r.array()?), r.octet()?);
            Some(NextHop {
                nh,
                ncc: octet & 0x07,
                spare: octet >> 3,
            })
        } else {
            None
        };
        value.subscribed_ue_ambr = if sambri {
            Some((r.u32()?, r.u32()?))
        } else {
            None
        };
        value.used_ue_ambr = if uambri {
            Some((r.u32()?, r.u32()?))
        } else {
            None
        };
        value.ue_network_capability = r.lv8()?;
        value.ms_network_capability = r.lv8()?;
        value.mei = r.lv8()?;
        value.access_restriction = AccessRestriction::of(r.octet()?);
        if osci {
            let s = r.octet()?;
            value.old_security_context = Some(OldSecurityContext {
                rlos: s & 0x40 != 0,
                ksi_asme: s >> 3 & 0x07,
                ncc: s & 0x07,
                kasme: Key(r.array()?),
                nh: if s & 0x80 != 0 {
                    Some(Key(r.array()?))
                } else {
                    None
                },
            });
        }
        let later = &mut value.later;
        later.voice_domain_preference = r.later(Reader::lv8)?;
        later.ue_radio_capability_for_paging = r.later(Reader::lv16)?;
        later.extended_access_restriction = r.later(Reader::lv8)?;
        later.ue_additional_security_capability = r.later(Reader::lv8)?;
        later.ue_nr_security_capability = r.later(Reader::lv8)?;
        later.apn_rate_control_statuses = r.later(Reader::lv16)?;
        later.core_network_restrictions = r.later(Reader::lv8)?;
        later.ue_radio_capability_id = r.later(Reader::lv8)?;
        later.octet_a = r.later(Reader::octet)?;
        value.extra = Octets::copied(r.0);
        Some(value)
    }

    fn encode(&self, out: &mut Vec<u8>) -> Result<(), Error> {
        let wide = |field: &'static str| Error::FieldLength { ie: IE, field };
        let fits = |value: u32, bits: u32, field: &'static str| match value >> bits {
            0 => Ok(()),
            _ => Err(wide(field)),
        };
        fits(self.security_mode.value().into(), 3, "Security Mode")?;
        fits(self.ksi_asme.into(), 3, "KSIASME")?;
        fits(
            self.used_nas_integrity.value().into(),
            3,
            "Used NAS integrity protection algorithm",
        )?;
        fits(self.used_nas_cipher.value().into(), 4, "Used NAS Cipher")?;
        fits(self.nas_downlink_count, 24, "NAS Downlink Count")?;
        fits(self.nas_uplink_count, 24, "NAS Uplink Count")?;
        let count = |n: usize, field| u8::try_from(n).ok().filter(|n| *n < 8).ok_or(wide(field));
        let quadruplets = count(self.quadruplets.len(), "Number of Quadruplet")?;
        let quintuplets = count(self.quintuplets.len(), "Number of Quintuplets")?;
        let lengths = [
            &self.ue_network_capability,
            &self.ms_network_capability,
            &self.mei,
        ];
        if lengths.iter().any(|octets| octets.len() > 255) {
            return Err(wide("a field after a length of one octet"));
        }
        out.push(
            self.security_mode.value() << 5
                | u8::from(self.nh.is_some()) << 4
                | u8::from(self.drx_parameter.is_some()) << 3
                | self.ksi_asme,
        );
        out.push(
            quintuplets << 5
                | quadruplets << 2
                | u8::from(self.used_ue_ambr.is_some()) << 1
                | u8::from(self.old_security_context.is_some()),
        );
        out.push(
            u8::from(self.subscribed_ue_ambr.is_some()) << 7
                | self.used_nas_integrity.value() << 4
                | self.used_nas_cipher.value(),
        );
        out.extend_from_slice(&self.nas_downlink_count.to_be_bytes()[1..]);
        out.extend_from_slice(&self.nas_uplink_count.to_be_bytes()[1..]);
        out.extend_from_slice(&self.kasme.0);
        for q in &self.quadruplets {
            out.extend_from_slice(&q.rand);
            lv8(&q.xres, out);
            lv8(&q.autn, out);
            out.extend_from_slice(&q.kasme.0);
        }
        for q in &self.quintuplets {
            out.extend_from_slice(&q.rand);
            lv8(&q.xres, out);
            out.extend_from_slice(&q.ck.0);
            out.extend_from_slice(&q.ik.0);
            lv8(&q.autn, out);
        }
        if let Some(drx) = &self.drx_parameter {
            out.extend_from_slice(drx);
        }
        if let Some(next) = &self.nh {
            fits(next.ncc.into(), 3, "NCC")?;
            fits(next.spare.into(), 5, "the spare bits of NCC")?;
            out.extend_from_slice(&next.nh.0);
            out.push(next.spare << 3 | next.ncc);
        }
        for (uplink, downlink) in [self.subscribed_ue_ambr, self.used_ue_ambr]
            .into_iter()
            .flatten()
        {
            out.extend_from_slice(&uplink.to_be_bytes());
            out.extend_from_slice(&downlink.to_be_bytes());
        }
        for octets in lengths {
            lv8(octets, out);
        }
        out.push(self.access_restriction.octet());
        if let Some(old) = &self.old_security_context {
            fits(old.ksi_asme.into(), 3, "old KSIASME")?;
            fits(old.ncc.into(), 3, "old NCC")?;
            out.push(
                u8::from(old.nh.is_some()) << 7
                    | u8::from(old.rlos) << 6
                    | old.ksi_asme << 3
                    | old.ncc,
            );
            out.extend_from_slice(&old.kasme.0);
            if let Some(nh) = &old.nh {
                out.extend_from_slice(&nh.0);
            }
        }
        self.later.encode(out)?;
        if !self.extra.is_empty() && self.later.octet_a.is_none() {
            return Err(Error::Inconsistent {
                ie: IE,
                field: "the octets of a later release",
                clause: "8.38",
            });
        }
        out.extend_from_slice(&self.extra);
        Ok(())
    }

    fn encoded_len(&self) -> usize {
        let mut out = Vec::new();
        let _ = self.encode(&mut out);
        out.len()
    }
}

impl Later {
    /// Write the fields that are there, in order, and those before the last of them that
    /// are not, as an empty field: a receiver reads the octets in order, and a field of a
    /// null length is one of no value (§8.38).
    fn encode(&self, out: &mut Vec<u8>) -> Result<(), Error> {
        let fields: [(Option<&Octets>, bool); 8] = [
            (self.voice_domain_preference.as_ref(), false),
            (self.ue_radio_capability_for_paging.as_ref(), true),
            (self.extended_access_restriction.as_ref(), false),
            (self.ue_additional_security_capability.as_ref(), false),
            (self.ue_nr_security_capability.as_ref(), false),
            (self.apn_rate_control_statuses.as_ref(), true),
            (self.core_network_restrictions.as_ref(), false),
            (self.ue_radio_capability_id.as_ref(), false),
        ];
        let written = match self.octet_a {
            Some(_) => fields.len(),
            None => fields
                .iter()
                .rposition(|(field, _)| field.is_some())
                .map_or(0, |last| last + 1),
        };
        let empty = Octets::default();
        for (field, wide) in &fields[..written] {
            let octets = field.unwrap_or(&empty);
            if octets.len() > if *wide { 65535 } else { 255 } {
                return Err(Error::FieldLength {
                    ie: IE,
                    field: "a field of a later release",
                });
            }
            match wide {
                true => lv16(octets, out),
                false => lv8(octets, out),
            }
        }
        if let Some(octet) = self.octet_a {
            out.push(octet);
        }
        Ok(())
    }
}

impl Rules for MmContextEpsSecurityContextQuadrupletsAndQuintuplets {
    fn check(&self, path: &Path<'_>, found: &mut Vec<Deviation>) {
        let rule = |what| Deviation::new(path, "8.38", DeviationKind::Rule(what));
        if self.security_mode != SecurityMode::EpsSecurityContextAndQuadruplets {
            found.push(rule(
                "the Security Mode of an EPS security context is not 4 (Table 8.38-1)",
            ));
        }
        if self.quadruplets.len() > 5 || self.quintuplets.len() > 5 {
            found.push(rule("more than 5 authentication vectors of a kind"));
        }
        let spare_nh = self.nh.is_some_and(|next| next.spare != 0);
        let spare_a = self.later.octet_a.is_some_and(|octet| octet > 0x07);
        if spare_nh || spare_a {
            found.push(Deviation::new(path, "8.38", DeviationKind::SpareBits));
        }
    }
}

impl MmContextEpsSecurityContextQuadrupletsAndQuintuplets {
    /// The names of its fields, as its tree writes them.
    pub(crate) const FIELDS: &'static [&'static str] = &[
        "Security Mode",
        "KSIASME",
        "Used NAS integrity protection algorithm",
        "Used NAS Cipher",
        "NAS Downlink Count",
        "NAS Uplink Count",
        "KASME",
        "Authentication Quadruplet",
        "Authentication Quintuplet",
        "DRX parameter",
        "NH",
        "NCC",
        "Uplink Subscribed UE AMBR",
        "Downlink Subscribed UE AMBR",
        "Uplink Used UE AMBR",
        "Downlink Used UE AMBR",
        "UE Network Capability",
        "MS Network Capability",
        "Mobile Equipment Identity (MEI)",
        "Access restriction data",
        "Old EPS Security Context",
        "Voice Domain Preference and UE's Usage Setting",
        "UE Radio Capability for Paging information",
        "Extended Access Restriction Data",
        "UE additional security capability",
        "UE NR security capability",
        "APN Rate Control Statuses",
        "Core Network Restrictions",
        "UE Radio Capability ID",
        "Octet a",
        "unknown octets",
    ];
    /// The members of its tree.
    pub(crate) const MEMBERS: &'static [&'static str] = Self::FIELDS;
    /// What its tree takes: an object of its fields.
    #[cfg(feature = "tree")]
    pub(crate) const TAKES: &'static oxirush_utils::tree::Takes =
        &oxirush_utils::tree::Takes::Members(None, &[]);
    /// Its layout, which the test spec does not compare yet.
    #[doc(hidden)]
    #[cfg(feature = "spec")]
    pub const LAYOUT: Option<&'static oxirush_utils::fields::Layout> = None;
}

#[cfg(feature = "tree")]
mod tree_form {
    use oxirush_utils::fields::FieldTree;
    use oxirush_utils::tree::{self, Members, Reading, Refused, Tree, Value};

    use super::*;

    const QUADRUPLET: &[&str] = &["RAND", "XRES", "AUTN", "KASME"];
    const QUINTUPLET: &[&str] = &["RAND", "XRES", "CK", "IK", "AUTN"];
    const OLD: &[&str] = &["RLOS", "old KSIASME", "old NCC", "old KASME", "old NH"];

    fn flags(restriction: AccessRestriction) -> Value {
        let mut members = Members::new();
        for (name, set) in AccessRestriction::NAMES.iter().zip(restriction.bits()) {
            if set {
                members.put(*name, Value::Bool(true));
            }
        }
        members.finish()
    }

    impl Tree for MmContextEpsSecurityContextQuadrupletsAndQuintuplets {
        const NAME: &'static str = IE;

        fn to_tree(&self) -> Value {
            let n = |value: u32| tree::unsigned(value.into());
            let mut m = Members::new();
            m.put("Security Mode", self.security_mode.to_tree());
            m.put("KSIASME", n(self.ksi_asme.into()));
            m.put(
                "Used NAS integrity protection algorithm",
                self.used_nas_integrity.to_tree(),
            );
            m.put("Used NAS Cipher", self.used_nas_cipher.to_tree());
            m.put("NAS Downlink Count", n(self.nas_downlink_count));
            m.put("NAS Uplink Count", n(self.nas_uplink_count));
            m.put("KASME", tree::octets(&self.kasme.0));
            if !self.quadruplets.is_empty() {
                let entries = self.quadruplets.iter().map(|q| {
                    let mut e = Members::new();
                    e.put("RAND", tree::octets(&q.rand));
                    e.put("XRES", tree::octets(&q.xres));
                    e.put("AUTN", tree::octets(&q.autn));
                    e.put("KASME", tree::octets(&q.kasme.0));
                    e.finish()
                });
                m.put("Authentication Quadruplet", Value::Array(entries.collect()));
            }
            if !self.quintuplets.is_empty() {
                let entries = self.quintuplets.iter().map(|q| {
                    let mut e = Members::new();
                    e.put("RAND", tree::octets(&q.rand));
                    e.put("XRES", tree::octets(&q.xres));
                    e.put("CK", tree::octets(&q.ck.0));
                    e.put("IK", tree::octets(&q.ik.0));
                    e.put("AUTN", tree::octets(&q.autn));
                    e.finish()
                });
                m.put("Authentication Quintuplet", Value::Array(entries.collect()));
            }
            m.put_optional(
                "DRX parameter",
                self.drx_parameter.map(|drx| tree::octets(&drx)),
            );
            if let Some(next) = &self.nh {
                m.put("NH", tree::octets(&next.nh.0));
                m.put("NCC", n(u32::from(next.spare) << 3 | u32::from(next.ncc)));
            }
            if let Some((uplink, downlink)) = self.subscribed_ue_ambr {
                m.put("Uplink Subscribed UE AMBR", n(uplink));
                m.put("Downlink Subscribed UE AMBR", n(downlink));
            }
            if let Some((uplink, downlink)) = self.used_ue_ambr {
                m.put("Uplink Used UE AMBR", n(uplink));
                m.put("Downlink Used UE AMBR", n(downlink));
            }
            m.put(
                "UE Network Capability",
                tree::octets(&self.ue_network_capability),
            );
            m.put(
                "MS Network Capability",
                tree::octets(&self.ms_network_capability),
            );
            m.put("Mobile Equipment Identity (MEI)", tree::octets(&self.mei));
            m.put("Access restriction data", flags(self.access_restriction));
            if let Some(old) = &self.old_security_context {
                let mut e = Members::new();
                if old.rlos {
                    e.put("RLOS", Value::Bool(true));
                }
                e.put("old KSIASME", n(old.ksi_asme.into()));
                e.put("old NCC", n(old.ncc.into()));
                e.put("old KASME", tree::octets(&old.kasme.0));
                e.put_optional("old NH", old.nh.map(|nh| tree::octets(&nh.0)));
                m.put("Old EPS Security Context", e.finish());
            }
            let l = &self.later;
            let later = [
                (
                    "Voice Domain Preference and UE's Usage Setting",
                    &l.voice_domain_preference,
                ),
                (
                    "UE Radio Capability for Paging information",
                    &l.ue_radio_capability_for_paging,
                ),
                (
                    "Extended Access Restriction Data",
                    &l.extended_access_restriction,
                ),
                (
                    "UE additional security capability",
                    &l.ue_additional_security_capability,
                ),
                ("UE NR security capability", &l.ue_nr_security_capability),
                ("APN Rate Control Statuses", &l.apn_rate_control_statuses),
                ("Core Network Restrictions", &l.core_network_restrictions),
                ("UE Radio Capability ID", &l.ue_radio_capability_id),
            ];
            for (name, octets) in later {
                m.put_optional(name, octets.as_ref().map(|octets| tree::octets(octets)));
            }
            m.put_optional("Octet a", l.octet_a.map(|octet| n(octet.into())));
            if !self.extra.is_empty() {
                m.put("unknown octets", tree::octets(&self.extra));
            }
            m.finish()
        }

        fn from_tree(value: &Value, reading: &mut Reading) -> Result<Self, Refused> {
            let read = reading.members(value, Self::FIELDS)?;
            let n =
                |reading: &mut Reading, value: &Value, most: u64| reading.unsigned(value, 0, most);
            let octets =
                |value: &Value, reading: &mut Reading| reading.octets(value).map(Octets::from);
            let small = |name, most, reading: &mut Reading| {
                reading.member(&read, name, |v, r| n(r, v, most))
            };
            let mut ctx = Self {
                security_mode: reading.member(&read, "Security Mode", SecurityMode::from_tree)?,
                ksi_asme: small("KSIASME", 7, reading)? as u8,
                used_nas_integrity: reading.member(
                    &read,
                    "Used NAS integrity protection algorithm",
                    NasIntegrity::from_tree,
                )?,
                used_nas_cipher: reading.member(&read, "Used NAS Cipher", NasCipher::from_tree)?,
                nas_downlink_count: small("NAS Downlink Count", 0xff_ffff, reading)? as u32,
                nas_uplink_count: small("NAS Uplink Count", 0xff_ffff, reading)? as u32,
                kasme: Key(reading.member(&read, "KASME", |v, r| r.octet_array(v))?),
                ..Self::default()
            };
            ctx.quadruplets = reading
                .optional(&read, "Authentication Quadruplet", |v, r| {
                    r.list(v, |e, r| {
                        let q = r.members(e, QUADRUPLET)?;
                        Ok(Quadruplet {
                            rand: r.member(&q, "RAND", |v, r| r.octet_array(v))?,
                            xres: r.member(&q, "XRES", octets)?,
                            autn: r.member(&q, "AUTN", octets)?,
                            kasme: Key(r.member(&q, "KASME", |v, r| r.octet_array(v))?),
                        })
                    })
                })?
                .unwrap_or_default();
            ctx.quintuplets = reading
                .optional(&read, "Authentication Quintuplet", |v, r| {
                    r.list(v, |e, r| {
                        let q = r.members(e, QUINTUPLET)?;
                        Ok(Quintuplet {
                            rand: r.member(&q, "RAND", |v, r| r.octet_array(v))?,
                            xres: r.member(&q, "XRES", octets)?,
                            ck: Key(r.member(&q, "CK", |v, r| r.octet_array(v))?),
                            ik: Key(r.member(&q, "IK", |v, r| r.octet_array(v))?),
                            autn: r.member(&q, "AUTN", octets)?,
                        })
                    })
                })?
                .unwrap_or_default();
            ctx.drx_parameter =
                reading.optional(&read, "DRX parameter", |v, r| r.octet_array(v))?;
            let nh = reading.optional(&read, "NH", |v, r| r.octet_array::<32>(v))?;
            let ncc = reading.optional(&read, "NCC", |v, r| n(r, v, 255))?;
            ctx.nh = match (nh, ncc) {
                (Some(nh), ncc) => {
                    let octet = ncc.unwrap_or(0) as u8;
                    Some(NextHop {
                        nh: Key(nh),
                        ncc: octet & 0x07,
                        spare: octet >> 3,
                    })
                }
                (None, None) => None,
                (None, Some(_)) => {
                    return Err(reading.refuse(tree::ErrorKind::Value, "an NCC without its NH"));
                }
            };
            let pair =
                |up: &'static str,
                 down: &'static str,
                 reading: &mut Reading|
                 -> Result<Option<(u32, u32)>, Refused> {
                    let up = reading.optional(&read, up, |v, r| n(r, v, u32::MAX.into()))?;
                    let down = reading.optional(&read, down, |v, r| n(r, v, u32::MAX.into()))?;
                    match (up, down) {
                        (Some(up), Some(down)) => Ok(Some((up as u32, down as u32))),
                        (None, None) => Ok(None),
                        _ => Err(reading
                            .refuse(tree::ErrorKind::Value, "an AMBR of one direction alone")),
                    }
                };
            ctx.subscribed_ue_ambr = pair(
                "Uplink Subscribed UE AMBR",
                "Downlink Subscribed UE AMBR",
                reading,
            )?;
            ctx.used_ue_ambr = pair("Uplink Used UE AMBR", "Downlink Used UE AMBR", reading)?;
            ctx.ue_network_capability = reading.member(&read, "UE Network Capability", octets)?;
            ctx.ms_network_capability = reading.member(&read, "MS Network Capability", octets)?;
            ctx.mei = reading.member(&read, "Mobile Equipment Identity (MEI)", octets)?;
            ctx.access_restriction = reading
                .optional(&read, "Access restriction data", |v, r| {
                    let f = r.members(v, &AccessRestriction::NAMES)?;
                    let mut octet = 0u8;
                    for (at, name) in AccessRestriction::NAMES.iter().enumerate() {
                        if r.optional(&f, name, |v, r| r.boolean(v))?.unwrap_or(false) {
                            octet |= 0x80 >> at;
                        }
                    }
                    Ok(AccessRestriction::of(octet))
                })?
                .unwrap_or_default();
            ctx.old_security_context =
                reading.optional(&read, "Old EPS Security Context", |v, r| {
                    let o = r.members(v, OLD)?;
                    Ok(OldSecurityContext {
                        rlos: r
                            .optional(&o, "RLOS", |v, r| r.boolean(v))?
                            .unwrap_or(false),
                        ksi_asme: r.member(&o, "old KSIASME", |v, r| n(r, v, 7))? as u8,
                        ncc: r.member(&o, "old NCC", |v, r| n(r, v, 7))? as u8,
                        kasme: Key(r.member(&o, "old KASME", |v, r| r.octet_array(v))?),
                        nh: r.optional(&o, "old NH", |v, r| r.octet_array(v))?.map(Key),
                    })
                })?;
            let mut later = |name| reading.optional(&read, name, octets);
            ctx.later = Later {
                voice_domain_preference: later("Voice Domain Preference and UE's Usage Setting")?,
                ue_radio_capability_for_paging: later(
                    "UE Radio Capability for Paging information",
                )?,
                extended_access_restriction: later("Extended Access Restriction Data")?,
                ue_additional_security_capability: later("UE additional security capability")?,
                ue_nr_security_capability: later("UE NR security capability")?,
                apn_rate_control_statuses: later("APN Rate Control Statuses")?,
                core_network_restrictions: later("Core Network Restrictions")?,
                ue_radio_capability_id: later("UE Radio Capability ID")?,
                octet_a: None,
            };
            ctx.later.octet_a = reading
                .optional(&read, "Octet a", |v, r| n(r, v, 255))?
                .map(|octet| octet as u8);
            ctx.extra = reading
                .optional(&read, "unknown octets", octets)?
                .unwrap_or_default();
            Ok(ctx)
        }
    }
}
