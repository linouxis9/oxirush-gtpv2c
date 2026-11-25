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

//! The codes, the spare bits and SMeNB of the identities of a User Location Information
//! (§8.21): what the encoder takes, what check() tells, and their tree.

use oxirush_gtpv2c::*;

/// The ULI of `octets`: the flags, then each identity after its PLMN.
fn uli(octets: &str) -> Uli {
    Uli::decode(&hex::decode(octets).unwrap()).unwrap()
}

fn encodes(uli: &Uli) -> bool {
    uli.encode(&mut Vec::new()).is_ok()
}

/// An ECGI has 4 spare bits before its ECI of 28 (Figure 8.21.5-1); a CGI none in its
/// codes of 16 bits each, and an RAC is one octet (§8.21.3).
#[test]
fn the_encoder_takes_the_codes_and_spare_bits_that_fit() {
    // ECGI: spare 0xf, ECI 0x1234567
    let mut ecgi = uli("1021f354f1234567");
    assert_eq!(ecgi.get("ECGI").unwrap().spare, 0x0f);
    assert!(encodes(&ecgi));
    let location = ecgi.locations[4].as_mut().unwrap();
    location.spare = 0x10;
    assert!(!encodes(&ecgi), "five spare bits");
    let location = ecgi.locations[4].as_mut().unwrap();
    location.spare = 0;
    location.code = 1 << 28;
    assert!(!encodes(&ecgi), "an ECI of 29 bits");
    // CGI: no spare bit
    let mut cgi = uli("0121f35411112222");
    assert!(encodes(&cgi));
    cgi.locations[0].as_mut().unwrap().spare = 1;
    assert!(!encodes(&cgi), "a spare bit of a CGI");
    // RAI: an RAC of 0xff, the largest
    let mut rai = uli("0421f354111144ff");
    rai.locations[2].as_mut().unwrap().second = 0xff;
    assert!(encodes(&rai));
}

/// Bit 8 of the first octet of an Extended Macro eNodeB ID is SMeNB, and bits 7 and 6
/// are spare (Figure 8.21.8-1).
#[test]
fn smenb_is_no_spare_bit() {
    // SMeNB set, ID 0x12345
    let smenb = uli("8021f354812345");
    assert_eq!(smenb.get("Extended Macro eNodeB ID").unwrap().spare, 0x04);
    assert!(smenb.deviations().is_empty());
    // A spare bit set
    let spare = uli("8021f354212345");
    let kinds: Vec<DeviationKind> = spare.deviations().into_iter().map(|d| d.kind).collect();
    assert_eq!(kinds, [DeviationKind::SpareBits]);
}

#[cfg(feature = "tree")]
#[test]
fn the_tree_has_smenb_and_the_spare_bits_apart() {
    use oxirush_gtpv2c::tree::{Tree, from_value};
    let smenb = uli("8021f354812345");
    let tree = smenb.to_tree();
    assert_eq!(
        tree.to_string(),
        r#"{"Extended Macro eNodeB ID":{"PLMN":"123-45","SMeNB":true,"Extended Macro eNodeB ID":74565}}"#
    );
    assert_eq!(from_value::<Uli>(&tree).unwrap(), smenb);
    // The bit of SMeNB in a Macro eNodeB ID is a spare bit
    let macro_ = uli("4021f354412345");
    let tree = macro_.to_tree();
    assert_eq!(
        tree.to_string(),
        r#"{"Macro eNodeB ID":{"PLMN":"123-45","Macro eNodeB ID":74565,"Spare":4}}"#
    );
    assert_eq!(from_value::<Uli>(&tree).unwrap(), macro_);
    // An ECI of 28 bits at most
    let largest = r#"{"ECGI":{"PLMN":"123-45","ECI":268435455}}"#;
    assert!(from_value::<Uli>(&largest.parse().unwrap()).is_ok());
    let wider = r#"{"ECGI":{"PLMN":"123-45","ECI":268435456}}"#;
    assert!(from_value::<Uli>(&wider.parse().unwrap()).is_err());
}
