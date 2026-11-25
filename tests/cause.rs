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

//! The spare bits of a Cause (§8.4): bits 8 to 4 of octet 6, and the spare bits of the
//! octet of the instance of the offending IE.

use oxirush_gtpv2c::*;

fn spare(octets: &str) -> bool {
    let cause = Cause::decode(&hex::decode(octets).unwrap()).unwrap();
    cause
        .deviations()
        .iter()
        .any(|d| d.kind == DeviationKind::SpareBits)
}

#[test]
fn the_spare_bits_of_a_cause() {
    // Request accepted, its flags as they are
    assert!(!spare("1000"));
    assert!(!spare("1007"));
    // Bit 4 of octet 6, with no offending IE
    assert!(spare("1008"));
    // An offending IE of instance 15, and one of a spare bit set
    assert!(!spare("10000a00000f"));
    assert!(spare("10000a000010"));
}
