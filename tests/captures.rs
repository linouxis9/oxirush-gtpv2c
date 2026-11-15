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

//! The GTPv2-C messages that Open5GS sent in lab runs (tests/captures/messages.tsv): each
//! decodes, encodes back to its octets, has a tree that reads back to the same octets,
//! and is checked, where only the deviations of `EXPECTED` are found.

use oxirush_gtpv2c::*;
#[cfg(feature = "tree")]
use oxirush_gtpv2c::{framing, paths, tree};

const MESSAGES: &str = include_str!("captures/messages.tsv");

fn messages() -> impl Iterator<Item = (&'static str, &'static str, Vec<u8>)> {
    MESSAGES
        .lines()
        .filter(|line| !line.starts_with('#'))
        .map(|line| {
            let cells: Vec<&str> = line.split('\t').collect();
            (cells[0], cells[1], hex::decode(cells[5]).expect("hex"))
        })
}

#[test]
fn each_message_is_byte_exact() {
    for (run, number, octets) in messages() {
        let message =
            Message::decode(&octets).unwrap_or_else(|error| panic!("{run} {number}: {error}"));
        assert_eq!(
            message.encode().unwrap(),
            octets,
            "{run} {number}: {message}"
        );
        #[cfg(feature = "tree")]
        {
            let tree = message.to_tree();
            let again = Message::from_tree(&tree)
                .unwrap_or_else(|error| panic!("{run} {number}: {error}: {tree}"));
            assert_eq!(again.encode().unwrap(), octets, "{run} {number}: {tree}");
            assert_eq!(again.check(), message.check(), "{run} {number}");
        }
    }
}

/// What the check finds in the captures, each a finding of the peer that sent it, by the
/// message type and the deviation as it prints, with how many times.
const EXPECTED: &[(usize, &str)] = &[
    // Open5GS's MME answers a Downlink Data Notification that it rejects ("UE already
    // re-attached", "Unable to page UE") with a Data Notification Delay, which only an
    // acceptance may have (§6.1.1): Open5GS v2.8.0, src/mme/mme-s11-build.c, which sets
    // it to 0 in each Downlink Data Notification Acknowledge
    (
        4,
        "Downlink Data Notification Acknowledge: Delay Value accompanies a rejection (TS 29.274 6.1.1), at /gtpv2c/Data Notification Delay",
    ),
];

#[test]
fn what_the_check_finds() {
    let mut found = std::collections::BTreeMap::<String, usize>::new();
    for (_, _, octets) in messages() {
        let message = Message::decode(&octets).unwrap();
        for deviation in message.check() {
            let name = message.body.name().unwrap_or("?");
            *found.entry(format!("{name}: {deviation}")).or_default() += 1;
        }
    }
    let found: Vec<(usize, String)> = found
        .into_iter()
        .map(|(what, count)| (count, what))
        .collect();
    let expected: Vec<(usize, String)> = EXPECTED
        .iter()
        .map(|(count, what)| (*count, what.to_string()))
        .collect();
    assert_eq!(found, expected);
}

/// Each IE of a message, at any depth, has the tree that `tree_of()` gives of its octets
/// in the place of its row, as the tree of the message has it.
#[cfg(feature = "tree")]
#[test]
fn each_ie_has_the_tree_of_its_octets() {
    fn walk(
        table: &'static Table,
        ies: framing::Ies<'_>,
        tree: &tree::Value,
        at: &str,
        count: &mut usize,
    ) {
        let mut seen = std::collections::BTreeMap::<(u16, u8), usize>::new();
        for ie in ies {
            let ie = ie.expect("framed");
            let key = (ie.ie_type, ie.spare << 4 | ie.instance);
            let nth = seen.entry(key).or_default();
            let Some(row) = table.row_of(key) else {
                continue;
            };
            let list = !matches!(row.roles[0].repeat, Repeat::Once);
            let path = match list {
                true => format!("{at}/{}/{nth}", row.name.replace('/', "~1")),
                false => format!("{at}/{}", row.name.replace('/', "~1")),
            };
            *nth += 1;
            let held = paths::select(tree, &path).unwrap_or_else(|error| panic!("{path}: {error}"));
            let info = ie_type_info(ie.ie_type).unwrap();
            assert_eq!(
                info.tree_of(ie.value, row.content).as_ref(),
                Some(held[0]),
                "{path}"
            );
            *count += 1;
            if let Some(content) = row.content {
                walk(content, ie.ies(), tree, &path, count);
            }
        }
    }
    let mut count = 0;
    for (run, number, octets) in messages() {
        let message = Message::decode(&octets).unwrap();
        let Some(table) = message.body.table() else {
            panic!("{run} {number}")
        };
        let (_, _, ies) = framing::message_ies(&octets).unwrap();
        walk(table, ies, &message.to_tree(), "/gtpv2c", &mut count);
    }
    assert!(count > 9_000, "{count} IEs");
}
