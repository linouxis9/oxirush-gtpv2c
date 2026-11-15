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

//! N26: a Forward Relocation Request of an AMF to an MME with its MM Context of an EPS
//! security context (§8.38, Figure 8.38-5), byte-exact through its tree, its keys out of
//! `Debug`.

#![cfg(feature = "tree")]

use oxirush_gtpv2c::*;
use oxirush_gtpv2c::{framing, paths, tree};

const KASME: &str = "000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f";
const NH: &str = "a0a1a2a3a4a5a6a7a8a9aaabacadaeafb0b1b2b3b4b5b6b7b8b9babbbcbdbebf";

fn request() -> tree::Value {
    tree!({
        "message": "Forward Relocation Request",
        "header": {"TEID": 0, "Sequence Number": 7},
        "ies": {
            "IMSI": "208930000000001",
            "Sender's F-TEID for Control Plane": {"V4": true, "Interface Type": 12, "TEID / GRE Key": 0x1001, "IPv4 address": "10.0.0.1"},
            "MME/SGSN/AMF UE MM Context (EPS Security Context, Quadruplets and Quintuplets)": {
                "Security Mode": 4,
                "KSIASME": 1,
                "Used NAS integrity protection algorithm": 2,
                "Used NAS Cipher": 0,
                "NAS Downlink Count": 5,
                "NAS Uplink Count": 3,
                "KASME": KASME,
                "NH": NH,
                "NCC": 2,
                "Uplink Subscribed UE AMBR": 100000,
                "Downlink Subscribed UE AMBR": 200000,
                "UE Network Capability": "f0f0c0c0",
                "MS Network Capability": "",
                "Mobile Equipment Identity (MEI)": "",
                "Access restriction data": {"NBNA": true},
                "Voice Domain Preference and UE's Usage Setting": "",
                "UE Radio Capability for Paging information": "",
                "Extended Access Restriction Data": "",
                "UE additional security capability": "",
                "UE NR security capability": "f0f0"
            }
        }
    })
}

#[test]
fn an_eps_mm_context_is_byte_exact_through_its_tree() {
    let message = Message::from_tree(&request()).unwrap();
    let octets = message.encode().unwrap();
    let decoded = Message::decode(&octets).unwrap();
    assert_eq!(decoded, message);
    assert_eq!(decoded.to_tree(), message.to_tree());
    assert_eq!(
        Message::from_tree(&decoded.to_tree())
            .unwrap()
            .encode()
            .unwrap(),
        octets
    );
    assert!(decoded.check().is_empty(), "{:?}", decoded.check());
    let Body::ForwardRelocationRequest(request) = &decoded.body else {
        panic!()
    };
    let context = request.mme_sgsn_amf_ue_mm_context_eps.as_ref().unwrap();
    assert_eq!(
        (
            context.ksi_asme,
            context.nas_downlink_count,
            context.nh.unwrap().1
        ),
        (1, 5, 2)
    );
    assert_eq!(
        context.later.ue_nr_security_capability.as_deref(),
        Some(&[0xf0, 0xf0][..])
    );
    // Octet 5: Security Mode 4, NHI, KSIASME 1; octet 6: no vectors, UAMBRI and OSCI
    // clear; octet 7: SAMBRI, EIA2, EEA0
    let (_, _, ies) = framing::message_ies(&octets).unwrap();
    let context = ies
        .map(Result::unwrap)
        .find(|ie| ie.ie_type == 107)
        .unwrap();
    assert_eq!(context.value[..3], [0x91, 0x00, 0xa0]);
    if std::env::var_os("GTPV2C_PRINT").is_some() {
        println!("{}", hex::encode(&octets));
    }
}

#[test]
fn its_keys_are_not_printed() {
    let message = Message::from_tree(&request()).unwrap();
    let debug = format!("{message:?}");
    assert!(
        !debug.contains("a0a1a2") && !debug.contains("160, 161"),
        "{debug}"
    );
    assert!(!debug.contains("0, 1, 2, 3, 4, 5, 6, 7, 8"), "{debug}");
}

#[test]
fn a_forward_relocation_request_without_an_mm_context() {
    let mut tree = request();
    paths::set(
        &mut tree,
        "/gtpv2c/MME~1SGSN~1AMF UE MM Context (EPS Security Context, Quadruplets and Quintuplets)",
        tree::Value::Null,
    )
    .ok();
    let ies = tree["ies"].as_object_mut().unwrap();
    ies.retain(|name, _| !name.contains("MM Context"));
    let found = Message::from_tree(&tree).unwrap().check();
    assert_eq!(found.len(), 1, "{found:?}");
    assert_eq!(
        found[0].kind,
        DeviationKind::MandatoryMissing { ie: (107, 0) }
    );
    assert_eq!(found[0].cause, Some(CauseValue::MandatoryIeMissing));
}
