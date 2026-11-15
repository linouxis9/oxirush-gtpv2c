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
use crate::fields::{Digits, Rest, SecretRest, U24, U40};

/// Nine octets of a field.
type Octets9 = [u8; 9];
/// Twelve octets of a field.
type Octets12 = [u8; 12];
use crate::ie::listed_value;
use crate::macros::gtpv2c_layout;
use crate::mm_context::MmContextEpsSecurityContextQuadrupletsAndQuintuplets;
use crate::tft::{EpsBearerLevelTrafficFlowTemplate, Tad};
use crate::uli::Uli;
use crate::values_hand::{Cause, FqCsid, Paa};
use oxirush_utils::fields::{Fqdn as Labels, PlmnId as Plmn};

gtpv2c_layout! {
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
    /// Trace Information (§8.31): the trace reference (the PLMN and the Trace ID) and what
    /// TS 32.422 says to trace, and the address of the trace collection entity.
    pub struct TraceInformation "8.31" {
        fields {
            plmn "PLMN": Plmn,
            trace_id "Trace ID": U24,
            triggering_events "Triggering Events": Octets9,
            ne_types "List of NE Types": u16,
            session_trace_depth "Session Trace Depth": u8,
            interfaces "List of Interfaces": Octets12,
            collection_entity "IP Address of Trace Collection Entity": Rest
        }
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

crate::macros::gtpv2c_bits! {
    /// Indication (§8.12): the flags of a request, in up to 10 octets, of which a sender
    /// of an earlier release sends fewer.
    pub struct Indication "8.12" octets 10 min 1 {
        daf "DAF": 0 8 1,
        dtf "DTF": 0 7 1,
        hi "HI": 0 6 1,
        dfi "DFI": 0 5 1,
        oi "OI": 0 4 1,
        isrsi "ISRSI": 0 3 1,
        israi "ISRAI": 0 2 1,
        sgwci "SGWCI": 0 1 1,
        sqci "SQCI": 1 8 1,
        uimsi "UIMSI": 1 7 1,
        cfsi "CFSI": 1 6 1,
        crsi "CRSI": 1 5 1,
        p "P": 1 4 1,
        pt "PT": 1 3 1,
        si "SI": 1 2 1,
        msv "MSV": 1 1 1,
        retloc "RetLoc": 2 8 1,
        pbic "PBIC": 2 7 1,
        srni "SRNI": 2 6 1,
        s6af "S6AF": 2 5 1,
        s4af "S4AF": 2 4 1,
        mbmdt "MBMDT": 2 3 1,
        israu "ISRAU": 2 2 1,
        ccrsi "CCRSI": 2 1 1,
        cprai "CPRAI": 3 8 1,
        arrl "ARRL": 3 7 1,
        ppof "PPOF": 3 6 1,
        ppon_ppei "PPON/PPEI": 3 5 1,
        ppsi "PPSI": 3 4 1,
        csfbi "CSFBI": 3 3 1,
        clii "CLII": 3 2 1,
        cpsr "CPSR": 3 1 1,
        nsi "NSI": 4 8 1,
        uasi "UASI": 4 7 1,
        dtci "DTCI": 4 6 1,
        bdwi "BDWI": 4 5 1,
        psci "PSCI": 4 4 1,
        pcri "PCRI": 4 3 1,
        aosi "AOSI": 4 2 1,
        aopi "AOPI": 4 1 1,
        roaai "ROAAI": 5 8 1,
        epcosi "EPCOSI": 5 7 1,
        cpopci "CPOPCI": 5 6 1,
        pmtsmi "PMTSMI": 5 5 1,
        s11tf "S11TF": 5 4 1,
        pnsi "PNSI": 5 3 1,
        unaccsi "UNACCSI": 5 2 1,
        wpmsi "WPMSI": 5 1 1,
        n5gsnn26 "5GSNN26": 6 8 1,
        reprefi "REPREFI": 6 7 1,
        n5gsiwk "5GSIWK": 6 6 1,
        eevrsi "EEVRSI": 6 5 1,
        ltemui "LTEMUI": 6 4 1,
        ltempi "LTEMPI": 6 3 1,
        enbcrsi "ENBCRSI": 6 2 1,
        tspcmi "TSPCMI": 6 1 1,
        csrmfi "CSRMFI": 7 8 1,
        mtedtn "MTEDTN": 7 7 1,
        mtedta "MTEDTA": 7 6 1,
        n5gnmi "N5GNMI": 7 5 1,
        n5gcnrs "5GCNRS": 7 4 1,
        n5gcnri "5GCNRI": 7 3 1,
        n5srhoi "5SRHOI": 7 2 1,
        ethpdn "ETHPDN": 7 1 1,
        nspusi "NSPUSI": 8 8 1,
        pgwrnsi "PGWRNSI": 8 7 1,
        rppcsi "RPPCSI": 8 6 1,
        pgwchi "PGWCHI": 8 5 1,
        sissme "SISSME": 8 4 1,
        nsenbi "NSENBI": 8 3 1,
        idfupf "IDFUPF": 8 2 1,
        emci "EMCI": 8 1 1,
        disrei "DISREI": 9 6 1,
        ppri "PPRI": 9 5 1,
        lapcosi "LAPCOSI": 9 4 1,
        ltemsai "LTEMSAI": 9 3 1,
        srtpi "SRTPI": 9 2 1,
        upipsi "UPIPSI": 9 1 1,
    }
}

crate::macros::gtpv2c_bits! {
    /// Bearer Flags (§8.32).
    pub struct BearerFlags "8.32" octets 1 min 1 {
        asi "ASI": 0 4 1, vind "Vind": 0 3 1, vb "VB": 0 2 1, ppc "PPC": 0 1 1,
    }
}

crate::macros::gtpv2c_bits! {
    /// UE Time Zone (§8.44): the time zone, as TS 24.008 clause 10.5.3.8 writes it, and the
    /// adjustment for daylight saving time.
    pub struct UeTimeZone "8.44" octets 2 min 2 {
        time_zone "Time Zone": 0 1 8, daylight_saving_time "Daylight Saving Time": 1 1 2,
    }
}

crate::macros::gtpv2c_bits! {
    /// Node Features (§8.83): the features that the sender supports.
    pub struct NodeFeatures "8.83" octets 1 min 1 {
        prn "PRN": 0 1 1, mabr "MABR": 0 2 1, ntsr "NTSR": 0 3 1, ciot "CIOT": 0 4 1,
        s1un "S1UN": 0 5 1, eth "ETH": 0 6 1, mtedt "MTEDT": 0 7 1, psset "PSSET": 0 8 1,
    }
}

crate::macros::gtpv2c_bits! {
    /// Allocation/Retention Priority (ARP) (§8.86).
    pub struct Arp "8.86" octets 1 min 1 {
        pci "PCI": 0 7 1, pl "PL": 0 3 4, pvi "PVI": 0 1 1,
    }
}

crate::macros::gtpv2c_bits! {
    /// EPC Timer (§8.87): a timer of a unit (Table 8.87-1) and a value.
    pub struct EpcTimer "8.87" octets 1 min 1 {
        timer_unit "Timer unit": 0 6 3, timer_value "Timer value": 0 1 5,
    }
}

crate::macros::gtpv2c_bits! {
    /// Signalling Priority Indication (§8.88): LAPI, the low access priority of the UE.
    pub struct SignallingPriorityIndication "8.88" octets 1 min 1 {
        lapi "LAPI": 0 1 1,
    }
}

include!("values_octets.rs");
