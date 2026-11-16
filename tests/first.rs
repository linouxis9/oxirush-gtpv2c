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

//! A first message of each kind: the Echo Request and its response, a Create Session
//! Request with a Bearer Context, an IE of an instance that its table does not have, and
//! one with its spare bits set, each byte-exact through its tree.

#[cfg(feature = "tree")]
use oxirush_gtpv2c::tree;
use oxirush_gtpv2c::*;

fn round_trip(octets: &[u8]) -> Message {
    let message = Message::decode(octets).expect("a message");
    assert_eq!(message.encode().unwrap(), octets, "the octets of {message}");
    #[cfg(feature = "tree")]
    {
        let tree = message.to_tree();
        let again = Message::from_tree(&tree).unwrap_or_else(|error| panic!("{error}: {tree}"));
        assert_eq!(again, message, "{tree}");
        assert_eq!(again.encode().unwrap(), octets, "the octets of {tree}");
    }
    message
}

#[test]
fn echo_request_and_response() {
    // Echo Request: Recovery 5
    let request = round_trip(&[
        0x40, 0x01, 0x00, 0x09, 0x00, 0x00, 0x01, 0x00, 0x03, 0x00, 0x01, 0x00, 0x05,
    ]);
    assert!(request.check().is_empty(), "{:?}", request.check());
    let Body::EchoRequest(echo) = &request.body else {
        panic!("{request}")
    };
    assert_eq!(
        echo.recovery.as_ref().map(|recovery| recovery.value),
        Some(5)
    );
    // Echo Response: Recovery 9
    let response = round_trip(&[
        0x40, 0x02, 0x00, 0x09, 0x00, 0x00, 0x01, 0x00, 0x03, 0x00, 0x01, 0x00, 0x09,
    ]);
    assert!(response.check().is_empty(), "{:?}", response.check());
}

#[test]
fn echo_request_without_recovery() {
    let request = round_trip(&[0x40, 0x01, 0x00, 0x04, 0x00, 0x00, 0x01, 0x00]);
    let found = request.check();
    assert_eq!(found.len(), 1, "{found:?}");
    assert_eq!(
        found[0].kind,
        DeviationKind::MandatoryMissing { ie: (3, 0) }
    );
    assert_eq!(found[0].clause, "7.7.6");
    // An Echo Request is not answered with a Cause (§7.1.1)
    assert_eq!(found[0].cause, None);
}

#[test]
fn an_instance_of_no_row_and_spare_bits_are_kept() {
    // Echo Request: Recovery 5, a Recovery of instance 1, and one whose spare bits are set
    let octets = [
        0x40, 0x01, 0x00, 0x13, 0x00, 0x00, 0x01, 0x00, //
        0x03, 0x00, 0x01, 0x00, 0x05, //
        0x03, 0x00, 0x01, 0x01, 0x06, //
        0x03, 0x00, 0x01, 0x10, 0x07,
    ];
    let message = round_trip(&octets);
    let Body::EchoRequest(echo) = &message.body else {
        panic!("{message}")
    };
    assert_eq!(echo.raw.len(), 2);
    assert_eq!((echo.raw[0].instance, echo.raw[1].instance), (1, 0x10));
    let found = message.check();
    let kinds: Vec<_> = found
        .iter()
        .map(|deviation| (deviation.clause, deviation.kind.clone()))
        .collect();
    assert!(
        kinds.contains(&("7.7.9", DeviationKind::Unexpected { ie: (3, 1) })),
        "{found:?}"
    );
    assert!(
        kinds.contains(&("8.2.1", DeviationKind::SpareBits)),
        "{found:?}"
    );
}

#[test]
fn a_create_session_request() {
    // Header with TEID 0, sequence 0x10; IMSI, RAT Type E-UTRAN, Sender F-TEID, APN,
    // and a Bearer Context to be created of an EBI 5 and a Bearer QoS
    let mut ies = Vec::new();
    let ie = |ies: &mut Vec<u8>, ie_type: u8, instance: u8, value: &[u8]| {
        ies.push(ie_type);
        ies.extend_from_slice(&(value.len() as u16).to_be_bytes());
        ies.push(instance);
        ies.extend_from_slice(value);
    };
    ie(
        &mut ies,
        1,
        0,
        &[0x02, 0x08, 0x93, 0x00, 0x00, 0x00, 0x00, 0xf1],
    );
    ie(&mut ies, 82, 0, &[6]);
    ie(
        &mut ies,
        87,
        0,
        &[0x8a, 0x00, 0x00, 0x00, 0x01, 127, 0, 0, 1],
    );
    ie(&mut ies, 71, 0, b"\x08internet");
    let mut bearer = Vec::new();
    ie(&mut bearer, 73, 0, &[5]);
    ie(
        &mut bearer,
        80,
        0,
        &[
            0x09, 9, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ],
    );
    ie(&mut ies, 93, 0, &bearer);
    let mut octets = vec![0x48, 32];
    octets.extend_from_slice(&((8 + ies.len()) as u16).to_be_bytes());
    octets.extend_from_slice(&[0, 0, 0, 0, 0, 0, 0x10, 0]);
    octets.extend_from_slice(&ies);
    let message = round_trip(&octets);
    let Body::CreateSessionRequest(request) = &message.body else {
        panic!("{message}")
    };
    assert_eq!(request.bearer_contexts_to_be_created.len(), 1);
    assert!(
        request.bearer_contexts_to_be_created[0]
            .eps_bearer_id
            .is_some()
    );
    let found = message.check();
    assert!(found.is_empty(), "{found:#?}");
}

#[test]
fn a_rejection_names_its_offending_ie() {
    // Create Session Response: Cause "Mandatory IE missing", of the RAT Type (82, instance 0)
    let octets = [
        0x48, 33, 0x00, 0x12, 0x00, 0x00, 0x10, 0x01, 0x00, 0x00, 0x02, 0x00, //
        0x02, 0x00, 0x06, 0x00, 70, 0x00, 82, 0x00, 0x00, 0x00,
    ];
    let message = round_trip(&octets);
    let Body::CreateSessionResponse(response) = &message.body else {
        panic!("{message}")
    };
    let cause = response.cause.as_ref().unwrap();
    assert_eq!(cause.value, CauseValue::MandatoryIeMissing);
    assert_eq!(
        cause.offending_ie.map(|ie| (ie.ie_type, ie.instance)),
        Some((82, 0))
    );
    assert!(message.check().is_empty(), "{:?}", message.check());
    #[cfg(feature = "tree")]
    assert_eq!(
        message.to_tree()["ies"]["Cause"].to_string(),
        r#"{"Cause value":"Mandatory IE missing","Offending IE":{"Type":82,"Instance":0}}"#
    );
}

#[test]
fn ies_kept_as_octets_keep_their_order_through_a_tree() {
    // Delete Bearer Command: a Bearer Context of instance 15 with its spare bits set, of
    // no row, then one of the row whose content does not frame, both kept as octets
    let octets = hex::decode(
        "4842002e11223644000001005d000fffffffffffffffff01000a5c000100025d000f0049000100065c000100005d000f0049",
    )
    .unwrap();
    let message = round_trip(&octets);
    #[cfg(feature = "tree")]
    {
        let read = Message::from_tree(&message.to_tree()).unwrap();
        assert_eq!(read.check(), message.check());
    }
}

#[cfg(feature = "tree")]
#[test]
fn the_conditions_that_a_message_shows() {
    let kinds = |tree: tree::Value| -> Vec<(String, &'static str, DeviationKind)> {
        let found = Message::from_tree(&tree).unwrap().check();
        found
            .into_iter()
            .map(|d| (d.path, d.clause, d.kind))
            .collect()
    };
    // An accepting Forward Relocation Response has the target's Sender F-TEID (7.3.2-1)
    let response = tree!({
        "message": "Forward Relocation Response",
        "header": {"TEID": 1, "Sequence Number": 1},
        "ies": {"Cause": {"Cause value": "Request accepted"}}
    });
    assert_eq!(
        kinds(response),
        [(
            "/gtpv2c/Sender's F-TEID for Control Plane".to_string(),
            "7.7.6",
            DeviationKind::ConditionalMissing { ie: (87, 0) }
        )]
    );
    // A PGW Back-Off Time of a Create Session Response that is no rejection for "APN
    // Congestion", and an Indication of no flag set (7.2.2-1)
    let response = tree!({
        "message": "Create Session Response",
        "header": {"TEID": 1, "Sequence Number": 1},
        "ies": {
            "Cause": {"Cause value": "Request accepted"},
            "Indication Flags": {},
            "Bearer Contexts created": [{"EPS Bearer ID": 5, "Cause": {"Cause value": "Request accepted"}}],
            "PGW Back-Off Time": {"Timer unit": 1, "Timer value": 3}
        }
    });
    let found = kinds(response);
    let excluded: Vec<&str> = found
        .iter()
        .filter(|(_, _, kind)| matches!(kind, DeviationKind::Excluded { .. }))
        .map(|(path, ..)| &path[..])
        .collect();
    assert_eq!(
        excluded,
        ["/gtpv2c/PGW Back-Off Time", "/gtpv2c/Indication Flags"],
        "{found:?}"
    );
}

#[test]
fn a_bearer_tft_by_its_packet_filters() {
    // Create new TFT, one bidirectional packet filter of precedence 16: a protocol
    // identifier 17 (UDP) and a single remote port 2152, then a parameter
    let octets = [
        0x31, 0x31, 0x10, 0x05, 0x30, 0x11, 0x50, 0x08, 0x68, 0x02, 0x01, 0x05,
    ];
    let tft = EpsBearerLevelTrafficFlowTemplate::decode(&octets).unwrap();
    assert_eq!((tft.operation, tft.filters.len()), (1, 1));
    assert_eq!(tft.filters[0].identifier, 0x31);
    assert_eq!(tft.parameters.as_ref().map(|p| p[0].identifier), Some(2));
    let mut again = Vec::new();
    tft.encode(&mut again).unwrap();
    assert_eq!(again, octets);
    assert!(tft.deviations().is_empty());
    #[cfg(feature = "tree")]
    assert_eq!(
        ie_type_info(84)
            .unwrap()
            .tree_of(&octets, None)
            .unwrap()
            .to_string(),
        r#"{"TFT operation code":1,"Packet filters":[{"Packet filter identifier":1,"Packet filter direction":3,"Packet filter evaluation precedence":16,"Packet filter contents":"3011500868"}],"Parameters":[{"Parameter identifier":2,"Parameter contents":"05"}]}"#
    );
}

#[test]
fn two_rows_of_one_key_are_one_field() {
    // Update Bearer Response: Cause, then an IP Address of instance 0, which is the
    // MME/S4-SGSN Identifier or the UE Local IP Address (Table 7.2.16-1, NOTE 2)
    let octets = hex::decode("4862001611223344000001000200020010004a000400c0a80001").unwrap();
    let message = round_trip(&octets);
    let Body::UpdateBearerResponse(response) = &message.body else {
        panic!("{message}")
    };
    assert_eq!(response.ip_address.len(), 1);
    // Its Bearer Contexts are left out
    let found = message.check();
    assert_eq!(found.len(), 1, "{found:?}");
    assert_eq!(
        found[0].kind,
        DeviationKind::MandatoryMissing { ie: (93, 0) }
    );
}

#[test]
fn a_cause_of_fewer_octets_than_an_offending_ie_encodes_back() {
    // Create Bearer Response: a Bearer Context whose Cause has one octet after its flags,
    // fewer than an offending IE, which a later release may write (found by fuzzing)
    let octets = hex::decode(
        "4823004611223344000001000200020010005d00180002000300100049310000055700090081ffffffff010101035d00180004000200100049000104065700090081ffffffff01010103",
    )
    .unwrap();
    round_trip(&octets);
}
