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

//! Every value that decodes encodes back to its octets, and through its tree: of each row of each table of
//! clause 7, at the level of the message and one level down, the values of the captures
//! and of go-gtp's vectors, each also with octets of a later release after it and one
//! octet short, and octets of no message (all 0, all 1, counting up) of each length up to
//! 40.

use std::collections::BTreeMap;

use oxirush_gtpv2c::*;

/// The values of each IE type that the captures and the vectors have, at any depth.
fn values() -> BTreeMap<u16, Vec<Vec<u8>>> {
    fn walk(ies: framing::Ies<'_>, out: &mut BTreeMap<u16, Vec<Vec<u8>>>) {
        for ie in ies.flatten() {
            let values = out.entry(ie.ie_type).or_default();
            if values.len() < 8 && !values.iter().any(|v| v == ie.value) {
                values.push(ie.value.to_vec());
            }
            if ie_type_info(ie.ie_type).is_some_and(|info| info.form == Form::Grouped) {
                walk(ie.ies(), out);
            }
        }
    }
    let mut out = BTreeMap::new();
    let files = [
        (include_str!("captures/messages.tsv"), 5),
        (include_str!("vectors/go-gtp.tsv"), 2),
    ];
    for (text, column) in files {
        for line in text.lines().filter(|line| !line.starts_with('#')) {
            let octets = hex::decode(line.split('\t').nth(column).unwrap()).unwrap();
            if let Ok((_, _, ies)) = framing::message_ies(&octets) {
                walk(ies, &mut out);
            }
        }
    }
    out
}

/// The contents to try of an IE of `ie_type`.
fn contents(ie_type: u16, values: &BTreeMap<u16, Vec<Vec<u8>>>) -> Vec<Vec<u8>> {
    let mut out = Vec::new();
    for value in values.get(&ie_type).into_iter().flatten() {
        out.push(value.clone());
        for tail in [
            &[0x01][..],
            &[0x01, 0x02],
            &[0x01, 0x02, 0x03],
            &[0x01, 0x02, 0x03, 0x04],
        ] {
            out.push([&value[..], tail].concat());
        }
        if !value.is_empty() {
            out.push(value[..value.len() - 1].to_vec());
        }
    }
    for length in 1..=40 {
        out.push(vec![0x00; length]);
        out.push(vec![0xff; length]);
        out.push((0..length as u8).collect());
    }
    out
}

fn ie(ie_type: u16, instance: u8, content: &[u8]) -> Vec<u8> {
    let mut out = vec![ie_type as u8];
    out.extend_from_slice(&(content.len() as u16).to_be_bytes());
    out.push(instance);
    out.extend_from_slice(content);
    out
}

fn message(message_type: u8, ies: &[u8]) -> Vec<u8> {
    let path = matches!(message_type, 1..=3);
    let mut out = vec![if path { 0x40 } else { 0x48 }, message_type];
    let header = if path { 4 } else { 8 };
    out.extend_from_slice(&((header + ies.len()) as u16).to_be_bytes());
    if !path {
        out.extend_from_slice(&[0, 0, 0, 1]);
    }
    out.extend_from_slice(&[0, 0, 1, 0]);
    out.extend_from_slice(ies);
    out
}

#[test]
fn every_value_that_decodes_encodes_back() {
    let values = values();
    let mut tried = 0;
    let mut problems = Vec::new();
    let mut try_one = |octets: Vec<u8>, what: String| {
        let Ok(message) = Message::decode(&octets) else {
            return;
        };
        tried += 1;
        #[cfg(feature = "tree")]
        {
            let tree = message.to_tree();
            match Message::from_tree(&tree).map(|read| read.encode()) {
                Ok(Ok(again)) if again == octets => {}
                Ok(Ok(_)) => problems.push(format!("{what}: other octets through the tree {tree}")),
                Ok(Err(error)) => {
                    problems.push(format!("{what}: {error:?} through the tree {tree}"))
                }
                Err(error) => problems.push(format!("{what}: {error} through the tree {tree}")),
            }
        }
        match message.encode() {
            Ok(again) if again == octets => {}
            Ok(_) => problems.push(format!("{what}: other octets of {}", hex::encode(&octets))),
            Err(error) => problems.push(format!("{what}: {error:?} of {}", hex::encode(&octets))),
        }
    };
    for info in MESSAGE_TYPES {
        for row in info.table.rows {
            let (ie_type, instance) = row.ie_type;
            for content in contents(ie_type, &values) {
                let what = format!("{} {}", info.table.id, row.name);
                try_one(
                    message(info.message_type, &ie(ie_type, instance, &content)),
                    what,
                );
            }
            // One level down: the IE in each row of the grouped IE of this row
            let Some(table) = row.content else { continue };
            for inner in table.rows {
                for content in contents(inner.ie_type.0, &values) {
                    let grouped = ie(
                        ie_type,
                        instance,
                        &ie(inner.ie_type.0, inner.ie_type.1, &content),
                    );
                    let what = format!("{} {} {}", info.table.id, row.name, inner.name);
                    try_one(message(info.message_type, &grouped), what);
                }
            }
        }
    }
    assert!(tried > 100_000, "{tried} messages");
    problems.sort();
    problems.dedup_by_key(|problem| problem.split(':').next().unwrap().to_string());
    assert!(
        problems.is_empty(),
        "{} values:\n{}",
        problems.len(),
        problems.join("\n")
    );
}
