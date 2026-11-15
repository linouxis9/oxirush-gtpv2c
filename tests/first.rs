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
