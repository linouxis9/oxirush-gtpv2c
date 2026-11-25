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

//! The messages of the tests of go-gtp v0.8.12 (tests/vectors/go-gtp.tsv): each decodes,
//! encodes back to its octets, and has a tree that reads back to the same octets; what
//! the check finds in them, which are written to test a codec and not to be sent, is
//! recorded in `EXPECTED`.

use oxirush_gtpv2c::*;

const VECTORS: &str = include_str!("vectors/go-gtp.tsv");

fn vectors() -> impl Iterator<Item = (&'static str, &'static str, Vec<u8>)> {
    VECTORS
        .lines()
        .filter(|line| !line.starts_with('#'))
        .map(|line| {
            let cells: Vec<&str> = line.split('\t').collect();
            (cells[0], cells[1], hex::decode(cells[2]).expect("hex"))
        })
}

#[test]
fn each_vector_is_byte_exact() {
    for (file, case, octets) in vectors() {
        let message =
            Message::decode(&octets).unwrap_or_else(|error| panic!("{file} {case}: {error}"));
        assert!(!matches!(message.body, Body::Other { .. }), "{file} {case}");
        assert_eq!(message.encode().unwrap(), octets, "{file} {case}");
        #[cfg(feature = "tree")]
        {
            let tree = message.to_tree();
            let again = Message::from_tree(&tree)
                .unwrap_or_else(|error| panic!("{file} {case}: {error}: {tree}"));
            assert_eq!(again.encode().unwrap(), octets, "{file} {case}: {tree}");
            assert_eq!(again.check(), message.check(), "{file} {case}");
        }
    }
}

/// What the check finds in each vector, as it prints: go-gtp's vectors test its codec,
/// and some are no messages to send. A Bearer Context of a Delete Bearer Command with two
/// Delay Values, of a Create Bearer Response with a Bearer QoS and no Cause, of an Update
/// Bearer Request with an F-TEID, a Release Access Bearers Request with Bearer Contexts,
/// a message of path management with a TEID (generic_test.go), a Create Session Request
/// of none of its mandatory IEs (header_test.go), and a ULI whose Macro and Extended Macro
/// eNodeB IDs are wider than their 20 and 21 bits, which go into the spare bits, and whose
/// RAC takes the two octets that §8.21.3 has the RAC and all 1's.
const EXPECTED: &[(&str, &str, &str)] = &[
    (
        "change-notification-req_test.go",
        "Normal",
        "spare bits are not zero (TS 29.274 8.21), at /gtpv2c/User Location Information (ULI)",
    ),
    (
        "change-notification-req_test.go",
        "Normal",
        "octet c+6 of an RAI is not all 1's (TS 29.274 8.21.3), at /gtpv2c/User Location Information (ULI)",
    ),
    (
        "create-bearer-res_test.go",
        "Normal",
        "the mandatory Cause is missing (TS 29.274 7.7.6), at /gtpv2c/Bearer Contexts/0/Cause",
    ),
    (
        "create-bearer-res_test.go",
        "Normal",
        "Bearer Level Quality of Service (Bearer QoS) is no IE of this table (TS 29.274 7.7.9), at /gtpv2c/Bearer Contexts/0/Bearer Level Quality of Service (Bearer QoS)",
    ),
    (
        "create-bearer-res_test.go",
        "Normal",
        "the mandatory Cause is missing (TS 29.274 7.7.6), at /gtpv2c/Bearer Contexts/1/Cause",
    ),
    (
        "create-bearer-res_test.go",
        "Normal",
        "Bearer Level Quality of Service (Bearer QoS) is no IE of this table (TS 29.274 7.7.9), at /gtpv2c/Bearer Contexts/1/Bearer Level Quality of Service (Bearer QoS)",
    ),
    (
        "delete-bearer-command_test.go",
        "Normal",
        "Delay Value is no IE of this table (TS 29.274 7.7.9), at /gtpv2c/Bearer Contexts/0/Delay Value/0",
    ),
    (
        "delete-bearer-command_test.go",
        "Normal",
        "Delay Value is no IE of this table (TS 29.274 7.7.9), at /gtpv2c/Bearer Contexts/0/Delay Value/1",
    ),
    (
        "delete-bearer-command_test.go",
        "Normal",
        "Delay Value is no IE of this table (TS 29.274 7.7.9), at /gtpv2c/Bearer Contexts/1/Delay Value/0",
    ),
    (
        "delete-bearer-command_test.go",
        "Normal",
        "Delay Value is no IE of this table (TS 29.274 7.7.9), at /gtpv2c/Bearer Contexts/1/Delay Value/1",
    ),
    (
        "delete-bearer-failure-indication_test.go",
        "Normal",
        "the mandatory Cause is missing (TS 29.274 7.7.6), at /gtpv2c/Bearer Context/0/Cause",
    ),
    (
        "delete-bearer-failure-indication_test.go",
        "Normal",
        "Delay Value is no IE of this table (TS 29.274 7.7.9), at /gtpv2c/Bearer Context/0/Delay Value/0",
    ),
    (
        "delete-bearer-failure-indication_test.go",
        "Normal",
        "Delay Value is no IE of this table (TS 29.274 7.7.9), at /gtpv2c/Bearer Context/0/Delay Value/1",
    ),
    (
        "delete-bearer-failure-indication_test.go",
        "Normal",
        "the mandatory Cause is missing (TS 29.274 7.7.6), at /gtpv2c/Bearer Context/1/Cause",
    ),
    (
        "delete-bearer-failure-indication_test.go",
        "Normal",
        "Delay Value is no IE of this table (TS 29.274 7.7.9), at /gtpv2c/Bearer Context/1/Delay Value/0",
    ),
    (
        "delete-bearer-failure-indication_test.go",
        "Normal",
        "Delay Value is no IE of this table (TS 29.274 7.7.9), at /gtpv2c/Bearer Context/1/Delay Value/1",
    ),
    (
        "delete-bearer-res_test.go",
        "Normal/CauseOnly",
        "the mandatory Cause is missing (TS 29.274 7.7.6), at /gtpv2c/Bearer Contexts/0/Cause",
    ),
    (
        "delete-bearer-res_test.go",
        "Normal/CauseOnly",
        "Fully Qualified Tunnel Endpoint Identifier (F-TEID) is no IE of this table (TS 29.274 7.7.9), at /gtpv2c/Bearer Contexts/0/Fully Qualified Tunnel Endpoint Identifier (F-TEID)",
    ),
    (
        "delete-bearer-res_test.go",
        "Normal/CauseOnly",
        "the mandatory Cause is missing (TS 29.274 7.7.6), at /gtpv2c/Bearer Contexts/1/Cause",
    ),
    (
        "delete-bearer-res_test.go",
        "Normal/CauseOnly",
        "Fully Qualified Tunnel Endpoint Identifier (F-TEID) is no IE of this table (TS 29.274 7.7.9), at /gtpv2c/Bearer Contexts/1/Fully Qualified Tunnel Endpoint Identifier (F-TEID)",
    ),
    (
        "generic_test.go",
        "Normal",
        "a message of path management has the T flag (TS 29.274 5.5.1), at /header/TEID",
    ),
    (
        "generic_test.go",
        "No-TEID",
        "a message of path management has the T flag (TS 29.274 5.5.1), at /header/TEID",
    ),
    (
        "header_test.go",
        "Normal",
        "the mandatory RAT Type is missing (TS 29.274 7.7.6), at /gtpv2c/RAT Type: the answer is \"Mandatory IE missing\"",
    ),
    (
        "header_test.go",
        "Normal",
        "the mandatory Fully Qualified Tunnel Endpoint Identifier (F-TEID) is missing (TS 29.274 7.7.6), at /gtpv2c/Sender F-TEID for Control Plane: the answer is \"Mandatory IE missing\"",
    ),
    (
        "header_test.go",
        "Normal",
        "the mandatory Access Point Name (APN) is missing (TS 29.274 7.7.6), at /gtpv2c/Access Point Name (APN): the answer is \"Mandatory IE missing\"",
    ),
    (
        "header_test.go",
        "Normal",
        "the mandatory Bearer Context is missing (TS 29.274 7.7.6), at /gtpv2c/Bearer Contexts to be created: the answer is \"Mandatory IE missing\"",
    ),
    (
        "modify-access-bearers-res_test.go",
        "Normal/CauseOnly",
        "the mandatory Cause is missing (TS 29.274 7.7.6), at /gtpv2c/Bearer Contexts modified/0/Cause",
    ),
    (
        "modify-access-bearers-res_test.go",
        "Normal/CauseOnly",
        "the mandatory Cause is missing (TS 29.274 7.7.6), at /gtpv2c/Bearer Contexts modified/1/Cause",
    ),
    (
        "modify-bearer-command_test.go",
        "Normal",
        "the mandatory EPS Bearer ID (EBI) is missing (TS 29.274 7.7.6), at /gtpv2c/Bearer Context/EPS Bearer ID: the answer is \"Mandatory IE missing\"",
    ),
    (
        "modify-bearer-command_test.go",
        "Normal",
        "Delay Value is no IE of this table (TS 29.274 7.7.9), at /gtpv2c/Bearer Context/Delay Value/0",
    ),
    (
        "modify-bearer-command_test.go",
        "Normal",
        "Delay Value is no IE of this table (TS 29.274 7.7.9), at /gtpv2c/Bearer Context/Delay Value/1",
    ),
    (
        "release-access-bearers-req_test.go",
        "Normal/WithIndication",
        "Bearer Context is no IE of this table (TS 29.274 7.7.9), at /gtpv2c/Bearer Context/0",
    ),
    (
        "release-access-bearers-req_test.go",
        "Normal/WithIndication",
        "Bearer Context is no IE of this table (TS 29.274 7.7.9), at /gtpv2c/Bearer Context/1",
    ),
    (
        "update-bearer-req_test.go",
        "Normal",
        "the mandatory Aggregate Maximum Bit Rate (AMBR) is missing (TS 29.274 7.7.6), at /gtpv2c/Aggregate Maximum Bit Rate (APN-AMBR): the answer is \"Mandatory IE missing\"",
    ),
    (
        "update-bearer-req_test.go",
        "Normal",
        "Fully Qualified Tunnel Endpoint Identifier (F-TEID) is no IE of this table (TS 29.274 7.7.9), at /gtpv2c/Bearer Contexts/0/Fully Qualified Tunnel Endpoint Identifier (F-TEID)",
    ),
    (
        "update-bearer-req_test.go",
        "Normal",
        "Fully Qualified Tunnel Endpoint Identifier (F-TEID) is no IE of this table (TS 29.274 7.7.9), at /gtpv2c/Bearer Contexts/1/Fully Qualified Tunnel Endpoint Identifier (F-TEID)",
    ),
    (
        "version-not-supported_test.go",
        "Normal/CauseOnly",
        "a message of path management has the T flag (TS 29.274 5.5.1), at /header/TEID",
    ),
];

#[test]
fn what_the_check_finds() {
    let mut found = Vec::new();
    for (file, case, octets) in vectors() {
        for deviation in Message::decode(&octets).unwrap().check() {
            found.push(format!(
                "    (\"{file}\", \"{case}\", \"{}\"),",
                deviation.to_string().replace('"', "\\\"")
            ));
        }
    }
    let expected: Vec<String> = EXPECTED
        .iter()
        .map(|(file, case, what)| {
            format!(
                "    (\"{file}\", \"{case}\", \"{}\"),",
                what.replace('"', "\\\"")
            )
        })
        .collect();
    assert!(found == expected, "\n{}", found.join("\n"));
}
