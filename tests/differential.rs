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

//! What comparing the crate with Wireshark's dissector of the same octets found: each test
//! is a value on which the two read a field otherwise, and what the clause says of it.
//! tests/wireshark/compare.py runs the comparison, and tests/wireshark/known.tsv lists the
//! differences that remain.

use oxirush_gtpv2c::*;

fn uli(octets: &str) -> Uli {
    let octets = hex::decode(octets).unwrap();
    let uli = Uli::decode(&octets).unwrap();
    let mut again = Vec::new();
    uli.encode(&mut again).unwrap();
    assert_eq!(again, octets, "byte-exact");
    uli
}

/// §8.21.3: "The Routing Area Code (RAC) consists of 2 octets. Only Octet c+5 contains the
/// RAC. Octet c+6 is coded as all 1's (11111111)." Wireshark reads the RAC from octet c+5,
/// as the clause does; the crate read it from both octets.
#[test]
fn the_rac_of_a_rai_is_its_first_octet() {
    // An RAI of PLMN 123-45, LAC 0x1111 and RAC 0x44
    let rai = uli("0421f354111144ff");
    let location = rai.get("RAI").unwrap();
    assert_eq!(
        (location.code, location.second, location.spare),
        (0x1111, 0x44, 0)
    );
    assert!(rai.deviations().is_empty());
    // Octet c+6 not all 1's, as go-gtp writes an RAC of 0x4444
    let rai = uli("0421f35411114444");
    assert_eq!(rai.get("RAI").unwrap().second, 0x44);
    let found: Vec<String> = rai.deviations().iter().map(|d| d.to_string()).collect();
    assert_eq!(
        found,
        ["octet c+6 of an RAI is not all 1's (TS 29.274 8.21.3), at "]
    );
    // An RAC is one octet
    let mut wide = uli("0421f354111144ff");
    wide.locations[2].as_mut().unwrap().second = 0x100;
    assert!(wide.encode(&mut Vec::new()).is_err());
    // An RAI of the spare of Location::default() has octet c+6 all 1's
    let mut built = Uli::default();
    built.locations[2] = Some(Location {
        plmn: rai.get("RAI").unwrap().plmn,
        code: 0x1111,
        second: 0x44,
        ..Location::default()
    });
    let mut octets = Vec::new();
    built.encode(&mut octets).unwrap();
    assert_eq!(hex::encode(octets), "0421f354111144ff");
}

/// The tree of an RAI has its RAC, and octet c+6 as written where it is not all 1's.
#[cfg(feature = "tree")]
#[test]
fn the_tree_of_a_rai_has_the_rac_alone() {
    use oxirush_gtpv2c::tree::{Tree, from_value};
    let rai = uli("0421f354111144ff");
    let tree = rai.to_tree();
    assert_eq!(
        tree.to_string(),
        r#"{"RAI":{"PLMN":"123-45","LAC":4369,"RAC":68}}"#
    );
    assert_eq!(from_value::<Uli>(&tree).unwrap(), rai);
    let filler = uli("0421f35411114444");
    let tree = filler.to_tree();
    assert_eq!(
        tree.to_string(),
        r#"{"RAI":{"PLMN":"123-45","LAC":4369,"RAC":68,"Spare":68}}"#
    );
    assert_eq!(from_value::<Uli>(&tree).unwrap(), filler);
    let wide = r#"{"RAI":{"PLMN":"123-45","LAC":1,"RAC":256}}"#;
    assert!(from_value::<Uli>(&wide.parse().unwrap()).is_err());
}

/// §7.7.7: "If the received value of the Length field and the actual length of the
/// extendable length IE are consistent, but the length is less than the number of fixed
/// octets defined for that IE, preceding the extended field(s), this shall be considered an
/// error". Table 8.1-1 gives the Indication 2 fixed octets; Wireshark finds one of one octet
/// of an older version, and the crate told nothing of it.
#[test]
fn an_extendable_ie_shorter_than_its_fixed_octets_is_told() {
    // A Create Session Request of go-gtp's vectors with Indication Flags of one octet
    let request = hex::decode("482000e711223344000001000100080021431532547698f04c00080021430521436587f94b00080021430521436587f956000d001821f354000121f354000001015300030021f35452000100064d000100a1570009008affffffff010101015700090187ffffffff010101024700110004736f6d650361706e076578616d706c65800001000063000100014f00050001020202027f000100014800080011111111222222225d001f0049000100055000160049ff11111111112222222222111111111122222222225d001f0049000100065000160049ff1111111111222222222211111111112222222222").unwrap();
    let found: Vec<(&str, DeviationKind)> = Message::decode(&request)
        .unwrap()
        .check()
        .into_iter()
        .map(|deviation| (deviation.clause, deviation.kind))
        .collect();
    let short = DeviationKind::FixedLength {
        ie: (77, 0),
        length: 1,
    };
    assert!(found.contains(&("7.7.7", short)), "{found:?}");
    // Two octets, and more, are the Indication as Table 8.1-1 has it
    let mut longer = request.clone();
    let at = 77;
    assert_eq!(&longer[at..at + 3], [77, 0, 1]);
    longer[at + 2] = 2;
    longer.insert(at + 5, 0);
    longer[3] += 1;
    let found = Message::decode(&longer).unwrap().check();
    assert!(
        found
            .iter()
            .all(|deviation| !matches!(deviation.kind, DeviationKind::FixedLength { .. })),
        "{found:?}"
    );
}

/// §8.4: "Cause is a variable length IE, which may have either of the following two
/// lengths values": 2, or 6 with the offending IE. Wireshark finds a Cause of 3 octets
/// malformed; the crate read it with an octet after the flags and told nothing of it.
#[test]
fn a_cause_of_neither_two_nor_six_octets_is_told() {
    let rule = DeviationKind::Rule("a Cause of neither 2 nor 6 octets");
    for (octets, told) in [
        ("1000", false),
        ("10000a000000", false),
        ("1000ff", true),
        ("1000ffffff", true),
        ("10000a000000ff", true),
    ] {
        let cause = Cause::decode(&hex::decode(octets).unwrap()).unwrap();
        let found: Vec<DeviationKind> = cause.deviations().into_iter().map(|d| d.kind).collect();
        assert_eq!(found.contains(&rule), told, "{octets}: {found:?}");
    }
}
