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

//! The values of the IEs of clause 8, one layout each: of the fields of its figure, or
//! of its octets where the crate does not split them yet.

use core::net::{Ipv4Addr, Ipv6Addr};

use crate::Octets;
use crate::enums::CauseValue;
use crate::fields::{Digits, Rest, SecretRest, U40};
use crate::ie::listed_value;
use crate::macros::gtpv2c_layout;
use oxirush_utils::fields::{Fqdn as Labels, PlmnId as Plmn};

gtpv2c_layout! {
    /// Cause (§8.4): the Cause value, the flags of the source of an error, and the IE that
    /// it concerns, which a rejection names after them (octets 7 to 10), kept in their
    /// place.
    pub struct Cause "8.4" {
        fields { value "Cause value": CauseValue }
        flags(u8) { pce "PCE": 3, bce "BCE": 2, cs "CS": 1 }
    }
    /// International Mobile Subscriber Identity (IMSI) (§8.3): its digits, as TBCD.
    pub struct Imsi "8.3" { fields { value "IMSI": Digits } }
    /// Access Point Name (APN) (§8.6): its labels, each after its length (TS 23.003 clause
    /// 9.1).
    pub struct Apn "8.6" { fields { value "Access Point Name (APN)": Labels } }
    /// Aggregate Maximum Bit Rate (AMBR) (§8.7): the APN-AMBR of each direction, in kbps.
    pub struct Ambr "8.7" { fields { uplink "APN-AMBR for uplink": u32, downlink "APN-AMBR for downlink": u32 } }
    /// Mobile Equipment Identity (MEI) (§8.10): an IMEI or an IMEISV, as TBCD.
    pub struct Mei "8.10" { fields { value "MEI": Digits } }
    /// MSISDN (§8.11): its digits, as TBCD.
    pub struct Msisdn "8.11" { fields { value "MSISDN": Digits } }
    /// Bearer Level Quality of Service (Bearer QoS) (§8.15): the ARP, the QCI, and the
    /// bitrates of each direction in kbps.
    pub struct BearerLevelQualityOfService "8.15" {
        flags(u8) { pci "PCI": 7, pl "PL": 3 / 4, pvi "PVI": 1 }
        qci "Label (QCI)": u8,
        mbr_uplink "Maximum bit rate for uplink": U40,
        mbr_downlink "Maximum bit rate for downlink": U40,
        gbr_uplink "Guaranteed bit rate for uplink": U40,
        gbr_downlink "Guaranteed bit rate for downlink": U40,
    }
    /// Serving Network (§8.18): the PLMN of the serving core network.
    pub struct ServingNetwork "8.18" { fields { value "PLMN": Plmn } }
    /// Fully Qualified TEID (F-TEID) (§8.22): a TEID or a GRE key with the addresses of its
    /// node, and the interface of the tunnel.
    pub struct FTeid "8.22" {
        flags(u8) { v4 "V4": 8, v6 "V6": 7, interface_type "Interface Type": 1 / 6 }
        teid "TEID / GRE Key": u32,
        ipv4 "IPv4 address": Ipv4Addr if v4,
        ipv6 "IPv6 address": Ipv6Addr if v6,
    }
    /// Fully Qualified Domain Name (FQDN) (§8.66): its labels, each after its length.
    pub struct Fqdn "8.66" { fields { value "FQDN": Labels } }
}

crate::macros::gtpv2c_bits! {
    /// EPS Bearer ID (EBI) (§8.8).
    pub struct Ebi "8.8" octets 1 min 1 {
        value "EPS Bearer ID (EBI)": 0 1 4,
    }
}

crate::macros::gtpv2c_bits! {
    /// PDN Type (§8.34): 1 IPv4, 2 IPv6, 3 IPv4v6, 4 Non-IP, 5 Ethernet.
    pub struct PdnType "8.34" octets 1 min 1 {
        value "PDN Type": 0 1 3,
    }
}

crate::macros::gtpv2c_bits! {
    /// Selection Mode (§8.58): how the APN was selected, of Table 8.58-1.
    pub struct SelectionMode "8.58" octets 1 min 1 {
        value "Selection Mode": 0 1 2,
    }
}

/// A value of one field, then the octets that a later release adds: a layout of that
/// field, whose tree is the field alone, and `new()` of it.
macro_rules! gtpv2c_field {
    ($( $(#[$meta:meta])* $ty:ident($t:ty) $field:literal $clause:literal; )*) => {$(
        gtpv2c_layout! {
            $(#[$meta])*
            pub struct $ty $clause { fields { value $field: $t } }
        }

        impl $ty {
            /// The value.
            pub fn new(value: $t) -> Self {
                Self { value, extra: Octets::default() }
            }
        }
    )*};
}

gtpv2c_field! {
    /// Recovery (§8.5): the Restart Counter of the sender.
    Recovery(u8) "Recovery (Restart Counter)" "8.5";
    /// RAT Type (§8.17), of Table 8.17-1.
    RatType(u8) "RAT Type" "8.17";
    /// Delay Value (§8.27): a delay in multiples of 50 milliseconds.
    DelayValue(u8) "Delay Value" "8.27";
    /// Charging ID (§8.29).
    ChargingId(u32) "Charging ID value" "8.29";
    /// Charging Characteristics (§8.30), as TS 32.251 writes them.
    ChargingCharacteristics(u16) "Charging Characteristics value" "8.30";
    /// Port Number (§8.52).
    PortNumber(u16) "Port Number" "8.52";
    /// APN Restriction (§8.57): its type, of Table 8.57-1.
    ApnRestriction(u8) "Restriction Type value" "8.57";
    /// Node Type (§8.65): 0 for an MME, 1 for an SGSN.
    NodeType(u8) "Node Type" "8.65";
}

include!("values_octets.rs");
