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

//! The paths of the tree of a message, which [`select`] reads and [`set`] edits.
//!
//! The tree of a message is on the engine of oxirush-utils ([`tree`](mod@crate::tree)): a
//! scope is an object whose members are its IEs by the names of its table, in the order of
//! the table; a row that is a list is a list, in the order of the wire. A path writes a
//! `/` of a name `~1`, as a JSON pointer does. An IE that the scope keeps as octets is
//! `{"octets": ..., "Instance": ...}` under the name of its row, of its type, or its
//! number; several of one type are a list.
//!
//! A tree writes the IEs of a scope in the order of the wire, each row where its first IE
//! is, and a message read from a tree keeps the order that the tree writes: decoding a
//! message, writing its tree and reading it back encodes the same octets, but where the
//! IEs of one row are apart on the wire, which a tree writes together.

use alloc::string::String;
use alloc::vec::Vec;

use oxirush_utils::tree::{self, Members, Reading, Refused, Takes, Value};

use crate::Gtpv2c;
use crate::ie::RawIe;
use crate::message::{MESSAGE_TYPES, MessageTypeInfo};
use crate::registry::{IE_TYPES, IeTypeInfo};
use crate::table::{Row, Table};

pub(crate) use oxirush_utils::tables::{
    TreeScope, field_tree, read_field, scope_from_tree, scope_to_tree,
};

/// The octets form of a raw IE: its content, its instance, the spare bits of its octet 4
/// where they are set, and a false Length.
fn raw_tree(ie: &RawIe) -> Value {
    let mut members = Members::new();
    members.put("octets", tree::octets(&ie.octets));
    members.put("Instance", tree::unsigned((ie.instance & 0x0f).into()));
    if ie.instance > 0x0f {
        members.put("Spare", tree::unsigned((ie.instance >> 4).into()));
    }
    members.put_optional(
        "length",
        ie.length.map(|length| tree::unsigned(length.into())),
    );
    members.finish()
}

fn read_raw(ie_type: u16, value: &Value, reading: &mut Reading) -> Result<RawIe, Refused> {
    let read = reading.members(value, &["octets", "Instance", "Spare", "length"])?;
    let octets = reading.member(&read, "octets", |value, reading| reading.octets(value))?;
    let instance = reading.optional(&read, "Instance", |value, reading| {
        reading.unsigned(value, 0, 15)
    })?;
    let spare = reading.optional(&read, "Spare", |value, reading| {
        reading.unsigned(value, 0, 15)
    })?;
    let length = reading.optional(&read, "length", |value, reading| {
        reading.unsigned(value, 0, 65535)
    })?;
    Ok(RawIe {
        ie_type,
        instance: (spare.unwrap_or(0) as u8) << 4 | instance.unwrap_or(0) as u8,
        length: length.map(|length| length as u16),
        octets: octets.into(),
    })
}

/// What an IE that a scope keeps as its octets takes, and each entry of a list of them.
const KEPT: Takes = Takes::Members(
    None,
    &[
        ("octets", &Takes::Octets(None)),
        ("Instance", &Takes::Number(15)),
        ("Spare", &Takes::Number(15)),
        ("length", &Takes::Number(u16::MAX as u64)),
    ],
);

/// The trees and the paths of GTPv2-C: `/gtpv2c` for the IEs of a message, an IE kept as
/// octets with its `Instance`.
impl oxirush_utils::tables::TreeProtocol for Gtpv2c {
    const ROOT: &'static str = "gtpv2c";
    const MESSAGE_MEMBERS: &'static [&'static str] = crate::message::forms::NAMES;
    const HEADER_MEMBERS: &'static [&'static str] = crate::message::forms::HEADER;
    const KEPT: &'static Takes = &KEPT;
    // An IE goes by its type and its instance: one of a row's type and another
    // instance is kept under the name of its type.
    const BY_TYPE: bool = false;
    // Senders write the IEs of a scope in orders of their own (Open5GS the Cause of a
    // Bearer Context after its F-TEID): a tree keeps the order of the wire.
    const WIRE_ORDER: bool = true;

    fn ie_types() -> &'static [IeTypeInfo] {
        IE_TYPES
    }

    fn message_types() -> &'static [MessageTypeInfo] {
        MESSAGE_TYPES
    }

    fn ie_type(key: (u16, u8)) -> u16 {
        key.0
    }

    fn key_of(ie_type: u16) -> (u16, u8) {
        (ie_type, 0)
    }

    fn holds_secret(ie_type: u16) -> bool {
        crate::ie::holds_secret(ie_type)
    }

    fn tree_name(key: (u16, u8), table: Option<&Table>) -> String {
        crate::tree_name(key, table)
    }

    fn raw_tree(ie: &RawIe) -> Value {
        raw_tree(ie)
    }

    fn read_raw(ie_type: u16, value: &Value, reading: &mut Reading) -> Result<RawIe, Refused> {
        read_raw(ie_type, value, reading)
    }
}

/// The error of `from_value` of a message in `tree`, at the path of the place that it
/// refuses, which [`select`] takes, and `/gtpv2c` for the IEs of the message.
pub(crate) fn located(tree: &Value, error: tree::Error) -> tree::Error {
    oxirush_utils::tables::located::<Gtpv2c>(tree, error)
}

/// The values at `path` in the tree of a message. A row of a table that a message or a
/// grouped IE lacks, an IE of no row by its number or the name of its type, as
/// [`Message::from_tree`](crate::Message::from_tree) reads it, and a member that a value
/// or the header lacks of those that its tree may have, select nothing; a name that is
/// none of them is an error. Below what is not there, and below an entry past the end of
/// a list, nothing is selected, whatever the path names: a misspelt name there is not
/// told. A value or a field whose tree is a scalar or an object takes the same names in
/// either form.
///
/// # Errors
///
/// Those of `oxirush_utils::tree::select_in`: `NoSuchMember` for a name that the value at
/// its place does not have and that its table or the members of its tree do not name
/// either.
pub fn select<'v>(tree: &'v Value, path: &str) -> Result<Vec<&'v Value>, tree::Error> {
    oxirush_utils::tables::select::<Gtpv2c>(tree, path)
}

/// The row of a table and the IE type of the IE at `path` in the tree of a message, or of
/// the list of them that a row is: the row that the tables of the message name at the
/// path, which the message may lack, as may the grouped IEs above it. `None` for a path of
/// the header, of a member of a value, or of an IE of no row.
///
/// ```
/// use oxirush_gtpv2c::{paths, tree};
///
/// let tree = tree!({"message": "Echo Request", "header": {"Sequence Number": 1}, "ies": {"Recovery": 7}});
/// let (row, info) = paths::row_at(&tree, "/gtpv2c/Sending Node Features")?.unwrap();
/// assert_eq!((row.name, info.ie_type), ("Sending Node Features", 152));
/// assert!(paths::row_at(&tree, "/header/Sequence Number")?.is_none());
/// # Ok::<(), tree::Error>(())
/// ```
///
/// # Errors
///
/// Those of [`select`].
pub fn row_at(
    tree: &Value,
    path: &str,
) -> Result<Option<(&'static Row, &'static IeTypeInfo)>, tree::Error> {
    oxirush_utils::tables::row_at::<Gtpv2c>(tree, path)
}

/// Give the place at `path` the value `value`: a path that [`select`] takes, with the rows
/// of grouped IEs and the lists that it lacks on the way added, an entry of a list at the
/// position of its length too. A member of the object of a value or a field that the tree
/// writes as a scalar writes the object, with the scalar as its member if it is one:
/// `{"value": 5, "Spare": 1}` from `5`. The tree is left as it was when the
/// edit fails.
///
/// A value that a message lacks is written whole: a path below it is refused
/// (`set(&mut tree, ".../Bearer Level QoS", tree!({...}))`). `set`
/// checks the names of the path, and not the value, which
/// [`Message::from_tree`](crate::Message::from_tree) reads.
///
/// # Errors
///
/// Those of `oxirush_utils::tree::set_in`: `NoSuchMember` for a name that [`select`]
/// refuses, the last of the path too, and `NothingSelected` for a path below what the tree
/// does not have.
pub fn set(tree: &mut Value, path: &str, value: impl Into<Value>) -> Result<(), tree::Error> {
    oxirush_utils::tables::set::<Gtpv2c>(tree, path, value)
}
