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

//! The crate against the fixtures of TS 29.274 V19.6.0 (tests/spec/extract.py), with what
//! errata.tsv reads otherwise: every IE type of Table 8.1-1, every message type of Table
//! 6.1-1 that has a table, and every table of clause 7, row by row.
//!
//! A row of the document of several lines, one per presence on other interfaces or
//! conditions, is one row of the crate: mandatory where each line is, of the weakest
//! presence of its lines otherwise, and conditional where one line is mandatory and
//! another is not. A note of a name ("(NOTE 29)") is no part of it, and the Private
//! Extension of each table, of the instance VS, is kept as octets.

use std::collections::BTreeMap;

use oxirush_gtpv2c::*;

fn records(text: &'static str) -> Vec<Vec<&'static str>> {
    text.lines()
        .filter(|line| !line.starts_with('#'))
        .map(|line| line.split('\t').collect())
        .collect()
}

const IE_TYPES_TSV: &str = include_str!("spec/ie-types.tsv");
const MESSAGE_TYPES_TSV: &str = include_str!("spec/message-types.tsv");
const TABLES_TSV: &str = include_str!("spec/tables.tsv");
const ERRATA_TSV: &str = include_str!("spec/errata.tsv");

fn errata(kind: &str) -> BTreeMap<&'static str, &'static str> {
    records(ERRATA_TSV)
        .into_iter()
        .filter(|line| line[0] == kind)
        .map(|line| match kind {
            "number" | "grouped" => (line[1], line[3]),
            _ => (line[2], line[3]),
        })
        .collect()
}

fn letters(text: &str) -> String {
    text.chars()
        .filter(char::is_ascii_alphanumeric)
        .map(|c| c.to_ascii_lowercase())
        .collect()
}

/// A name without its notes.
fn without_notes(name: &str) -> String {
    let mut out = name.to_string();
    while let Some(at) = out.find(" (NOTE").or_else(|| out.find(" (See NOTE")) {
        let end = out[at..].find(')').map_or(out.len(), |end| at + end + 1);
        out.replace_range(at..end, "");
    }
    out.trim().to_string()
}

/// The name without its last parenthesis, and the parenthesis: `IMSI` of
/// `International Mobile Subscriber Identity (IMSI)`.
fn short(name: &str) -> (String, Option<String>) {
    match name
        .trim_end()
        .strip_suffix(')')
        .and_then(|rest| rest.rfind(" (").map(|at| (at, rest)))
    {
        Some((at, rest)) => (rest[..at].to_string(), Some(rest[at + 2..].to_string())),
        None => (name.to_string(), None),
    }
}

fn report(problems: Vec<String>) {
    assert!(
        problems.is_empty(),
        "{} differences:\n{}",
        problems.len(),
        problems.join("\n")
    );
}

#[test]
fn ie_types() {
    let names = errata("ie-name");
    let mut problems = Vec::new();
    for line in records(IE_TYPES_TSV) {
        let (Ok(number), name) = (line[0].parse::<u16>(), line[1]) else {
            continue;
        };
        // 254 says that an extended type follows (§8.2.1A): framing, whose IEs the crate
        // keeps as octets.
        if name.starts_with("Reserved") || name.starts_with("Spare") || number == 254 {
            continue;
        }
        let name = names.get(name).copied().unwrap_or(name);
        let Some(info) = ie_type_info(number) else {
            problems.push(format!("IE type {number} {name}: not in the crate"));
            continue;
        };
        if info.name != name {
            problems.push(format!(
                "IE type {number}: {:?}, and the document {name:?}",
                info.name
            ));
        }
        let reference = line[2];
        let form = match reference.split('/').next().map(str::trim) {
            Some("Fixed Length") => Some(Form::Fixed),
            Some("Variable Length" | "Variable") => Some(Form::Variable),
            Some("Extendable") => Some(Form::Extendable),
            _ => None,
        };
        let grouped = info.form == Form::Grouped;
        if let Some(form) = form
            && !grouped
            && info.form != form
        {
            problems.push(format!(
                "IE type {number}: {:?}, and the document {reference}",
                info.form
            ));
        }
        let fixed = line[3].trim().parse::<u16>().ok();
        if !grouped && info.fixed != fixed {
            problems.push(format!(
                "IE type {number}: fixed {:?}, and the document {:?}",
                info.fixed, line[3]
            ));
        }
    }
    let listed = records(IE_TYPES_TSV)
        .iter()
        .filter_map(|line| line[0].parse::<u16>().ok())
        .collect::<Vec<_>>();
    for info in IE_TYPES {
        if !listed.contains(&info.ie_type) {
            problems.push(format!(
                "IE type {} {}: not in Table 8.1-1",
                info.ie_type, info.name
            ));
        }
    }
    report(problems);
}

/// The IE type of a cell of a table: by the name of a type, its abbreviation, or the name
/// without it, and by the aliases of errata.tsv.
fn ie_type_of(cell: &str) -> Option<u16> {
    let aliases = errata("ie-type");
    let cell = without_notes(cell);
    if let Some(number) = aliases.get(&cell[..]) {
        return number.parse().ok();
    }
    let wanted = [letters(&cell), letters(&short(&cell).0)];
    IE_TYPES.iter().map(|info| info.ie_type).find(|&number| {
        let name = ie_type_info(number).unwrap().name;
        let (long, abbreviation) = short(name);
        let known = [
            Some(letters(name)),
            abbreviation.map(|a| letters(&a)),
            Some(letters(&long)),
        ];
        wanted
            .iter()
            .any(|wanted| known.iter().flatten().any(|known| known == wanted))
    })
}

struct Expected {
    name: String,
    key: (u16, u8),
    presences: Vec<&'static str>,
    list: bool,
}

/// The tables of the document, by the number that the crate gives them, with the IE type
/// of a grouped IE's and the rows.
fn expected_tables() -> Vec<(String, String, Option<u16>, Vec<Expected>)> {
    let numbers = errata("number");
    let grouped = errata("grouped");
    let mut tables: Vec<(String, String, Option<u16>, Vec<Expected>)> = Vec::new();
    for line in records(TABLES_TSV) {
        match line[0] {
            "table" => {
                let prelude = grouped.get(line[1]).copied().unwrap_or(line[3]);
                let number = numbers.get(line[1]).copied().unwrap_or(line[1]);
                tables.push((
                    number.to_string(),
                    line[2].to_string(),
                    prelude.parse().ok(),
                    Vec::new(),
                ));
            }
            "row" | "also" => {
                let (name, presence, cell, instance, several) =
                    (line[2], line[3], line[4], line[5], line[6]);
                if instance == "VS" {
                    continue;
                }
                let rows = &mut tables.last_mut().unwrap().3;
                let name = without_notes(name);
                let ie_type = ie_type_of(cell).unwrap_or_else(|| panic!("no IE type of {cell:?}"));
                let instance = match instance.parse::<u8>() {
                    Ok(instance) => instance,
                    Err(_) => rows
                        .last()
                        .map(|row: &Expected| row.key.1)
                        .expect("an instance"),
                };
                match rows.last_mut() {
                    Some(last) if last.name == name && last.key == (ie_type, instance) => {
                        last.presences.push(presence);
                        last.list |= several == "several";
                    }
                    _ => rows.push(Expected {
                        name,
                        key: (ie_type, instance),
                        presences: vec![presence],
                        list: several == "several",
                    }),
                }
            }
            _ => {}
        }
    }
    tables
}

fn presence_of(presences: &[&str]) -> Presence {
    let rank = |letter: &str| match letter {
        "M" => 0,
        "C" => 1,
        "CO" => 2,
        _ => 3,
    };
    if presences.iter().all(|letter| *letter == "M") {
        return Presence::Mandatory;
    }
    if presences.contains(&"M") {
        return Presence::Conditional;
    }
    match presences.iter().max_by_key(|letter| rank(letter)).copied() {
        Some("C") => Presence::Conditional,
        Some("CO") => Presence::ConditionalOptional,
        _ => Presence::Optional,
    }
}

/// Every table of the crate by its number.
fn crate_tables() -> BTreeMap<&'static str, &'static Table> {
    let mut all = BTreeMap::new();
    for info in MESSAGE_TYPES {
        all.insert(info.table.id, info.table);
    }
    for info in IE_TYPES {
        for table in info.tables {
            all.insert(table.id, *table);
        }
    }
    all
}

#[test]
fn tables() {
    let mut problems = Vec::new();
    let tables = crate_tables();
    let mut seen = Vec::new();
    for (number, caption, grouped, rows) in expected_tables() {
        let Some(table) = tables.get(&number[..]) else {
            problems.push(format!("Table {number} ({caption}): not in the crate"));
            continue;
        };
        seen.push(table.id);
        if let Some(ie_type) = grouped
            && table.code != ie_type
        {
            problems.push(format!(
                "Table {number}: of IE type {}, and the document {ie_type}",
                table.code
            ));
        }
        let names: Vec<&str> = table.rows.iter().map(|row| row.name).collect();
        let wanted: Vec<&str> = rows.iter().map(|row| &row.name[..]).collect();
        if names != wanted {
            problems.push(format!(
                "Table {number}: rows {names:?}\n  and the document {wanted:?}"
            ));
            continue;
        }
        for (row, expected) in table.rows.iter().zip(&rows) {
            let role = &row.roles[0];
            let presence = presence_of(&expected.presences);
            if row.ie_type != expected.key {
                problems.push(format!(
                    "Table {number} {}: {:?}, and the document {:?}",
                    row.name, row.ie_type, expected.key
                ));
            }
            if role.presence != presence {
                problems.push(format!(
                    "Table {number} {}: {:?}, and the document {:?}",
                    row.name, role.presence, expected.presences
                ));
            }
            let list = !matches!(role.repeat, Repeat::Once);
            if list != expected.list {
                problems.push(format!(
                    "Table {number} {}: a list {list}, and the document {}",
                    row.name, expected.list
                ));
            }
            if let Some(content) = row.content
                && content.code != expected.key.0
            {
                problems.push(format!(
                    "Table {number} {}: content of type {}",
                    row.name, content.code
                ));
            }
        }
    }
    for id in tables.keys() {
        if !seen.contains(id) {
            problems.push(format!("Table {id}: not in the document"));
        }
    }
    report(problems);
}

#[test]
fn message_types() {
    let mut problems = Vec::new();
    let document: BTreeMap<u8, &str> = records(MESSAGE_TYPES_TSV)
        .into_iter()
        .filter_map(|line| Some((line[0].parse().ok()?, line[1])))
        .collect();
    for info in MESSAGE_TYPES {
        let Some(name) = document.get(&info.message_type) else {
            problems.push(format!(
                "message type {}: not in Table 6.1-1",
                info.message_type
            ));
            continue;
        };
        let name = name.split(" (").next().unwrap().trim();
        if letters(name) != letters(info.name())
            && !letters(info.name()).starts_with(&letters(name))
        {
            problems.push(format!(
                "message type {}: {:?}, and Table 6.1-1 {name:?}",
                info.message_type,
                info.name()
            ));
        }
        let initial = records(MESSAGE_TYPES_TSV)
            .iter()
            .any(|line| line[0] == info.message_type.to_string() && line[2].trim() == "X");
        if is_initial(info.message_type) != initial {
            problems.push(format!(
                "message type {}: initial {}",
                info.message_type, !initial
            ));
        }
    }
    report(problems);
}
