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

//! What `check()` finds: each way in which a message breaks TS 29.274, with its path in
//! the tree of the message, the clause that it breaks and the Cause with which a receiver
//! answers a request for it, its offending IE by its type and instance.

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;
use core::fmt;

use crate::enums::CauseValue;
use crate::registry::ie_type_info;
use crate::table::{Interfaces, Presence, Row, Scope, Table};
use oxirush_utils::tables::TableFinding;

pub(crate) use oxirush_utils::fields::Path;
pub(crate) use oxirush_utils::tables::{as_read, condition_absent, condition_missing, taken};

/// One deviation: where, which clause, what, and the Cause of the answer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Deviation {
    /// Where the deviation is, as a path of the tree of the message, which `paths::select`
    /// takes: the IE, at its position in the list of its row (`/gtpv2c/Bearer Contexts to
    /// be created/0/EPS Bearer ID`); the row, for what the row has as a whole; a field of
    /// the header (`/header/TEID`); `/message` for the message type; and `/gtpv2c` for the
    /// IEs together. A name with a `/` has it written `~1`.
    pub path: String,
    /// The clause or the table that the message breaks.
    pub clause: &'static str,
    /// What is wrong.
    pub kind: DeviationKind,
    /// The Cause with which the receiver of a request answers it (§7.7, Table 8.4-1):
    /// `None` for a deviation that the receiver ignores, and in a response.
    pub cause: Option<CauseValue>,
    /// The IE type and instance of the IE at `path`, when the deviation is in its value.
    pub(crate) ie: Option<(u16, u8)>,
}

/// What a deviation is. An IE goes by its type and instance, as the Cause of an answer
/// names it (§8.4).
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum DeviationKind {
    /// A mandatory IE is not there (§7.7.6).
    MandatoryMissing {
        /// Its IE type and instance.
        ie: (u16, u8),
    },
    /// A mandatory IE is there, and its value does not decode as its type (§7.7.8).
    MandatoryIncorrect {
        /// Its IE type and instance.
        ie: (u16, u8),
    },
    /// A conditional IE is not there where the scope shows that its condition holds
    /// (§7.7.6).
    ConditionalMissing {
        /// Its IE type and instance.
        ie: (u16, u8),
    },
    /// A mandatory IE, or a conditional one where the scope shows that its condition
    /// holds, of a null length.
    NullLength {
        /// Its IE type and instance.
        ie: (u16, u8),
    },
    /// A conditional IE that is there, and whose value does not decode as its type: a
    /// verifiable conditional IE of a semantically invalid value (§7.7.8).
    ConditionalIncorrect {
        /// Its IE type and instance.
        ie: (u16, u8),
    },
    /// An optional or conditional-optional IE whose value does not decode as its type:
    /// the receiver takes it as absent (§7.7.8).
    Incorrect {
        /// Its IE type and instance.
        ie: (u16, u8),
    },
    /// An IE that is there more times than its table allows: the receiver handles the
    /// first (§7.7.10).
    Repeated {
        /// Its IE type and instance.
        ie: (u16, u8),
    },
    /// An IE of a known type whose instance its table does not have, or whose type it does
    /// not have: the receiver discards it (§7.7.9).
    Unexpected {
        /// Its IE type and instance.
        ie: (u16, u8),
    },
    /// An IE that is there where the condition beside it, which the scope shows, has it
    /// absent.
    Excluded {
        /// Its IE type and instance.
        ie: (u16, u8),
    },
    /// An IE in a response with a rejection Cause, which only the Cause and the IEs that
    /// §6.1.1 lists may accompany.
    WithRejection {
        /// Its IE type and instance.
        ie: (u16, u8),
    },
    /// An IE whose Length is not the length of its content (§7.7.7).
    InvalidLength {
        /// Its IE type and instance.
        ie: (u16, u8),
    },
    /// An IE of a fixed length (Table 8.1-1) that has fewer octets (§7.7.7).
    FixedLength {
        /// Its IE type and instance.
        ie: (u16, u8),
        /// Its length.
        length: usize,
    },
    /// An IE type that Table 8.1-1 does not have, which the receiver discards (§7.7.9).
    UnknownType {
        /// Its IE type and instance.
        ie: (u16, u8),
    },
    /// A message type that Table 6.1-1 does not have, which the receiver discards
    /// (§7.7.4).
    UnknownMessage {
        /// The message type.
        message_type: u8,
    },
    /// Spare bits that are not zero, which the receiver ignores (§8.2.1).
    SpareBits,
    /// A value that its table has as spare, or does not have: the receiver ignores it.
    SpareValue {
        /// The value.
        value: u32,
    },
    /// A value that its table reserves: the receiver takes the IE as incorrect (§7.7.8).
    ReservedValue {
        /// The value.
        value: u32,
    },
    /// A value that its table has the sender not send, and the receiver read as another.
    ReadAs {
        /// The value.
        value: u32,
        /// The value that the receiver takes.
        read: u32,
    },
    /// A field that is there where the flags of its IE do not have it, or the reverse, or
    /// whose octets would read as another value.
    Inconsistent {
        /// The field.
        field: &'static str,
    },
    /// A rule of the clause, in words.
    Rule(&'static str),
}

impl Deviation {
    /// A deviation at `path`, that the receiver ignores.
    pub fn new(path: impl fmt::Display, clause: &'static str, kind: DeviationKind) -> Self {
        Self {
            path: format!("{path}"),
            clause,
            kind,
            cause: None,
            ie: None,
        }
    }

    /// The IE type and instance that the Cause of the answer names as its offending IE
    /// (§8.4): the IE that is missing, or whose value is incorrect.
    pub fn offending_ie(&self) -> Option<(u16, u8)> {
        let ie = match self.kind {
            DeviationKind::MandatoryMissing { ie }
            | DeviationKind::MandatoryIncorrect { ie }
            | DeviationKind::ConditionalMissing { ie }
            | DeviationKind::ConditionalIncorrect { ie }
            | DeviationKind::InvalidLength { ie }
            | DeviationKind::FixedLength { ie, .. } => Some(ie),
            _ => self.ie,
        };
        self.cause.and(ie)
    }
}

impl fmt::Display for Deviation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = |(ie_type, instance): (u16, u8)| match ie_type_info(ie_type) {
            Some(info) if instance == 0 => alloc::string::ToString::to_string(info.name),
            Some(info) => format!("{} of instance {instance}", info.name),
            None => format!("IE type {ie_type}"),
        };
        match &self.kind {
            DeviationKind::MandatoryMissing { ie } => {
                write!(f, "the mandatory {} is missing", name(*ie))
            }
            DeviationKind::MandatoryIncorrect { ie } => {
                write!(f, "the mandatory {} does not decode", name(*ie))
            }
            DeviationKind::ConditionalMissing { ie } => {
                write!(f, "{} is missing where its condition holds", name(*ie))
            }
            DeviationKind::ConditionalIncorrect { ie } => {
                write!(f, "the conditional {} does not decode", name(*ie))
            }
            DeviationKind::NullLength { ie } => {
                write!(
                    f,
                    "{} has a null length where it needs its value",
                    name(*ie)
                )
            }
            DeviationKind::Incorrect { ie } => {
                write!(f, "{} does not decode, and is taken as absent", name(*ie))
            }
            DeviationKind::Repeated { ie } => {
                write!(
                    f,
                    "{} is present more times than its table allows",
                    name(*ie)
                )
            }
            DeviationKind::Unexpected { ie } => write!(f, "{} is no IE of this table", name(*ie)),
            DeviationKind::Excluded { ie } => {
                write!(f, "{} is present but its condition is not met", name(*ie))
            }
            DeviationKind::WithRejection { ie } => {
                write!(f, "{} accompanies a rejection", name(*ie))
            }
            DeviationKind::InvalidLength { ie } => write!(
                f,
                "{} has a Length that is not the length of its content",
                name(*ie)
            ),
            DeviationKind::FixedLength { ie, length } => {
                write!(
                    f,
                    "{} has {length} octets, fewer than its fixed length",
                    name(*ie)
                )
            }
            DeviationKind::UnknownType { ie } => {
                write!(f, "IE type {} is in no table of Release 19", ie.0)
            }
            DeviationKind::UnknownMessage { message_type } => {
                write!(
                    f,
                    "message type {message_type} is in no table of Release 19"
                )
            }
            DeviationKind::SpareBits => write!(f, "spare bits are not zero"),
            DeviationKind::SpareValue { value } => write!(f, "{value} is a spare value"),
            DeviationKind::ReservedValue { value } => write!(f, "{value} is a reserved value"),
            DeviationKind::ReadAs { value, read } => {
                write!(f, "{value} is no value to send, and is read as {read}")
            }
            DeviationKind::Inconsistent { field } => {
                write!(
                    f,
                    "{field} disagrees with the flags or the other fields of its IE"
                )
            }
            DeviationKind::Rule(rule) => write!(f, "{rule}"),
        }?;
        write!(f, " (TS 29.274 {}), at {}", self.clause, self.path)?;
        if let Some(cause) = self.cause {
            write!(f, ": the answer is \"{cause}\"")?;
        }
        Ok(())
    }
}

/// What a value checks of itself, at its place in a message: what [`Check`] gives.
pub trait Rules {
    /// Add what is wrong with the value to `found`, at `path`.
    fn check(&self, path: &Path<'_>, found: &mut Vec<Deviation>) {
        let _ = (path, found);
    }

    /// Add what is wrong with the value in a row of the table `table`, by its number,
    /// which the value alone does not show.
    fn check_placed(&self, table: &'static str, path: &Path<'_>, found: &mut Vec<Deviation>) {
        let _ = (table, path, found);
    }
}

/// What a value checks of itself: its flags, its fields and its values. A grouped IE
/// checks its table.
pub trait Check: Rules {
    /// How the value breaks its clause: each deviation at its path below the value, `""`
    /// for the value itself.
    fn deviations(&self) -> Vec<Deviation> {
        let mut found = Vec::new();
        self.check(&Path::Root(""), &mut found);
        found
    }
}

impl<T: Rules> Check for T {}

/// Where a check is: how the receiver takes the mandatory IEs of the scope (§6.1.1: those
/// of a conditional grouped IE as conditional, those of a conditional-optional or
/// optional one as optional), whether the receiver answers it, and the Cause of a
/// response that rejects. The checks of the tables of oxirush-utils take it through
/// Gtpv2c's `Checking`; it is no part of the crate's interface.
#[doc(hidden)]
#[derive(Debug, Clone, Copy)]
pub struct Context {
    pub(crate) view: Presence,
    /// In a request that the receiver answers with a Cause.
    pub(crate) request: bool,
    /// The Cause of a response that rejects the request, at the message level.
    pub(crate) rejected: Option<CauseValue>,
}

impl Context {
    /// The context of a grouped IE checked on its own, as the receiver of a response
    /// reads it: GTPv2-C has no interfaces in its tables.
    pub(crate) fn on(_interfaces: Interfaces) -> Self {
        Self::message()
    }

    /// The message level of a message, which no receiver answers.
    pub(crate) fn message() -> Self {
        Self {
            view: Presence::Mandatory,
            request: false,
            rejected: None,
        }
    }

    /// The context of the IEs of a grouped IE of the row of presence `presence` (§6.1.1).
    pub(crate) fn within(self, presence: Presence) -> Self {
        let view = match (self.view, presence) {
            (Presence::Optional, _) | (_, Presence::Optional) => Presence::Optional,
            (Presence::ConditionalOptional, _) | (_, Presence::ConditionalOptional) => {
                Presence::ConditionalOptional
            }
            (Presence::Mandatory, Presence::Mandatory) => Presence::Mandatory,
            _ => Presence::Conditional,
        };
        Self {
            view,
            rejected: None,
            ..self
        }
    }

    /// The Cause of an answer to a mandatory IE of the scope that is missing, or that is
    /// there and incorrect (§7.7.6, §7.7.8).
    pub(crate) fn cause(self, missing: bool) -> Option<CauseValue> {
        match (self.request, self.view, missing) {
            (false, ..) | (_, Presence::Optional | Presence::ConditionalOptional, _) => None,
            (true, Presence::Mandatory, true) => Some(CauseValue::MandatoryIeMissing),
            (true, Presence::Conditional, true) => Some(CauseValue::ConditionalIeMissing),
            (true, _, false) => Some(CauseValue::MandatoryIeIncorrect),
        }
    }

    /// The Cause of an answer to a request of an IE whose Length is not the length of its
    /// content (§7.7.7).
    pub(crate) fn invalid_length(self) -> Option<CauseValue> {
        self.request.then_some(CauseValue::InvalidLength)
    }

    /// The Cause of an answer to a conditional IE of the scope that is missing where its
    /// condition holds (§7.7.6).
    pub(crate) fn conditional_cause(self) -> Option<CauseValue> {
        match (self.request, self.view) {
            (true, Presence::Mandatory | Presence::Conditional) => {
                Some(CauseValue::ConditionalIeMissing)
            }
            _ => None,
        }
    }
}

/// What the table macro writes: the check of each value of the fields of a scope.
pub(crate) trait Checked {
    fn check_values(&self, path: &Path<'_>, context: Context, found: &mut Vec<Deviation>);
}

/// The path of the IEs of a message, which `/gtpv2c` names.
pub(crate) const ROOT: &str = "/gtpv2c";

/// The deviation of a message type that Table 6.1-1 does not have (§7.7.4).
pub(crate) fn unknown_message(message_type: u8) -> Deviation {
    Deviation::new(
        "/message",
        "7.7.4",
        DeviationKind::UnknownMessage { message_type },
    )
}

/// A finding of a field of oxirush-utils, as a deviation that the receiver ignores: spare
/// bits by §8.2.1.
impl oxirush_utils::fields::Deviate for Deviation {
    fn found(
        path: &Path<'_>,
        clause: Option<&'static str>,
        finding: oxirush_utils::fields::Finding,
    ) -> Self {
        use oxirush_utils::fields::Finding;
        let kind = match finding {
            Finding::SpareBits => DeviationKind::SpareBits,
            Finding::SpareValue { value } => DeviationKind::SpareValue { value },
            Finding::ReservedValue { value } => DeviationKind::ReservedValue { value },
            Finding::ReadAs { value, read } => DeviationKind::ReadAs { value, read },
            Finding::Inconsistent { field } => DeviationKind::Inconsistent { field },
            Finding::Rule(rule) => DeviationKind::Rule(rule),
        };
        Deviation::new(path, clause.unwrap_or("8.2.1"), kind)
    }
}

/// Check a value of a row of the table `table` of a scope: itself, in its place, and its
/// length against Table 8.1-1.
pub(crate) fn check_field<T: crate::ie::Listed + crate::ie::IeType + crate::table::Content>(
    value: &T,
    key: (u16, u8),
    table: &'static str,
    path: &Path<'_>,
    presence: Presence,
    context: Context,
    found: &mut Vec<Deviation>,
) {
    let from = found.len();
    value.check_in(path, context.within(presence), found);
    value.check_placed(table, path, found);
    // The fixed octets that Table 8.1-1 gives a fixed-length IE, and an extendable one
    // before its extended fields: more are a later release's, which the receiver discards,
    // and fewer are an error (§7.7.7).
    let fixed = T::FIXED.or_else(|| crate::ie_type_info(T::IE_TYPE).and_then(|info| info.fixed));
    if let Some(fixed) = fixed {
        let length = value.encoded_len();
        if length < usize::from(fixed) {
            found.push(Deviation::new(
                path,
                "7.7.7",
                DeviationKind::FixedLength { ie: key, length },
            ));
        }
    }
    if found.len() > from {
        let grouped = T::TABLE.is_some();
        value_deviations(&mut found[from..], path, grouped, key, presence, context);
    }
}

/// The deviations of the value of an IE itself, not those of the IEs of a grouped one,
/// which their own rows answer: each gets the IE, and a value that is semantically
/// invalid the Cause of a mandatory or verifiable conditional IE (§7.7.8). A spare value
/// or spare bits are ignored.
#[cold]
#[inline(never)]
fn value_deviations(
    found: &mut [Deviation],
    path: &Path<'_>,
    grouped: bool,
    key: (u16, u8),
    presence: Presence,
    context: Context,
) {
    let here = format!("{path}");
    let own = |at: &str| {
        let below = at.strip_prefix(&here[..]);
        below.is_some_and(|below| below.is_empty() || !grouped && below.starts_with('/'))
    };
    for deviation in found.iter_mut().filter(|deviation| own(&deviation.path)) {
        deviation.ie = Some(key);
        let invalid = matches!(
            deviation.kind,
            DeviationKind::ReservedValue { .. }
                | DeviationKind::FixedLength { .. }
                | DeviationKind::Inconsistent { .. }
                | DeviationKind::Rule(_)
        );
        if invalid && matches!(presence, Presence::Mandatory | Presence::Conditional) {
            deviation.cause = context.cause(false);
        }
    }
}

/// Check the values of the row at `at` of a scope: the row, then each value, at its
/// position in the tree, which the IEs of the row that the scope keeps as octets share.
#[inline(always)]
pub(crate) fn check_field_values<
    T: crate::ie::Listed + crate::ie::IeType + crate::table::Content,
    F: crate::table::FieldValues<T>,
    S: Scope,
>(
    scope: &S,
    at: usize,
    field: &F,
    path: &Path<'_>,
    context: Context,
    found: &mut Vec<Deviation>,
) {
    let table = S::table();
    let key = table.rows[at].ie_type;
    oxirush_utils::tables::check_values(
        scope,
        at,
        field.values(),
        path,
        context,
        found,
        |value, here, presence, found| {
            check_field(value, key, table.id, here, presence, context, found)
        },
    );
}

/// Check a scope against its table: its mandatory IEs, the IEs that it keeps as octets
/// (repeated, incorrect, of no row of the table, of no release), then each row that has
/// values, and each value.
pub(crate) fn check_scope<S: Scope + Checked>(
    scope: &S,
    path: &Path<'_>,
    context: Context,
    found: &mut Vec<Deviation>,
) {
    oxirush_utils::tables::check_scope(
        scope,
        path,
        context,
        found,
        |scope, path, context, found| scope.check_values(path, context, found),
    );
}

/// The IE types that a response with a rejection Cause may have besides the Cause (§6.1.1):
/// Protocol Configuration Options, Recovery, User Location Information, Load Control
/// Information, Overload Control Information, Bearer Context and Local Distinguished
/// Name.
const WITH_REJECTION: [u16; 7] = [78, 3, 86, 181, 180, 93, 151];

/// The IE types that the response of a table may have besides with a rejection Cause
/// (§6.1.1): the F-Cause of a Forward Relocation Response, the DL low priority traffic
/// Throttling and the IMSI of a Downlink Data Notification Acknowledge, the PGW Back-Off
/// Time (an EPC Timer) of a Create Session Response, the IMSI and the MEI of a Change
/// Notification Response, the IMSI of a Context Response and of a UE Registration Query
/// Response, and the RAN/NAS Cause of a Create, Update and Delete Bearer Response.
const WITH_TABLE: [(&str, u16); 11] = [
    ("7.3.2-1", 119),
    ("7.2.11.2-1", 154),
    ("7.2.11.2-1", 1),
    ("7.2.2-1", 156),
    ("7.3.15-1", 1),
    ("7.3.15-1", 75),
    ("7.3.6-1", 1),
    ("7.3.22-1", 1),
    ("7.2.4-1", 172),
    ("7.2.16-1", 172),
    ("7.2.10.2-1", 172),
];

/// Whether a response with a rejection Cause may have the IEs of a row.
fn with_rejection(table: &str, row: &Row) -> bool {
    let ie_type = row.ie_type.0;
    ie_type == 2
        || WITH_REJECTION.contains(&ie_type)
        || WITH_TABLE
            .iter()
            .any(|(id, of)| *id == table && *of == ie_type)
}

/// The IEs of a row that a message needs: one of a mandatory row, but of a rejection, in
/// which only the Cause follows its row (§6.1.1).
#[inline]
fn mandatory(row: &Row, context: Context) -> usize {
    let needed = row
        .roles
        .iter()
        .filter(|role| role.presence == Presence::Mandatory)
        .count();
    match context.rejected {
        Some(_) => needed.min(usize::from(row.ie_type.0 == 2)),
        None => needed,
    }
}

/// The IE whose Length runs past the grouped IE of `row` whose content is `octets`
/// (§7.7.7): the first whose Length runs past the others, or the grouped IE itself where
/// fewer octets than an IE header are left. `None` for an IE of another row, and for IEs
/// that frame.
fn unframed_within(row: &Row, octets: &[u8]) -> Option<(u16, u8)> {
    row.content?;
    crate::framing::ies(octets).find_map(|ie| match ie.err()? {
        crate::ie::IeFault::Length { ie_type, .. } => Some((ie_type, 0)),
        crate::ie::IeFault::HeaderCut { .. } => Some(row.ie_type),
    })
}

/// The rules of GTPv2-C that the checks of the tables of oxirush-utils take: the clauses
/// and the Causes of clause 7.7, and the IEs of a rejection (§6.1.1).
impl oxirush_utils::tables::Checking for crate::Gtpv2c {
    type Deviation = Deviation;
    type Context = Context;

    #[inline]
    fn interfaces(_context: Context) -> Interfaces {
        Interfaces::ALL
    }

    fn deviation(
        path: &dyn fmt::Display,
        table: &'static Table,
        finding: TableFinding<(u16, u8)>,
        context: Context,
    ) -> Deviation {
        let (clause, kind, cause) = match finding {
            TableFinding::MandatoryMissing(ie) => (
                "7.7.6",
                DeviationKind::MandatoryMissing { ie },
                context.cause(true),
            ),
            TableFinding::MandatoryIncorrect(ie) => (
                "7.7.8",
                DeviationKind::MandatoryIncorrect { ie },
                context.cause(false),
            ),
            TableFinding::ConditionalIncorrect(ie) => (
                "7.7.8",
                DeviationKind::ConditionalIncorrect { ie },
                context.cause(false),
            ),
            TableFinding::Incorrect(ie) => ("7.7.8", DeviationKind::Incorrect { ie }, None),
            TableFinding::ConditionalMissing(ie) => (
                "7.7.6",
                DeviationKind::ConditionalMissing { ie },
                context.conditional_cause(),
            ),
            TableFinding::NullLength(ie) => (table.id, DeviationKind::NullLength { ie }, None),
            TableFinding::Repeated(ie) => ("7.7.10", DeviationKind::Repeated { ie }, None),
            TableFinding::Excluded(ie) => (table.id, DeviationKind::Excluded { ie }, None),
            TableFinding::NotOnInterface(ie) => ("7.7.9", DeviationKind::Unexpected { ie }, None),
            TableFinding::WithRejection(ie) => ("6.1.1", DeviationKind::WithRejection { ie }, None),
            TableFinding::InvalidLength(ie) => (
                "7.7.7",
                DeviationKind::InvalidLength { ie },
                context.invalid_length(),
            ),
        };
        let mut deviation = Deviation::new(path, clause, kind);
        deviation.cause = cause;
        deviation
    }

    #[inline]
    fn mandatory(row: &Row, context: Context) -> usize {
        mandatory(row, context)
    }

    #[inline]
    fn rejected(table: &'static str, row: &Row, _count: usize, context: Context) -> bool {
        context.rejected.is_some() && !with_rejection(table, row)
    }

    #[inline]
    fn check_row_more(
        _table: &'static str,
        _row: &Row,
        _count: usize,
        _path: &Path<'_>,
        _context: Context,
        _found: &mut Vec<Deviation>,
    ) {
    }

    fn of_no_row(path: &str, ie: &crate::ie::RawIe) -> Option<Deviation> {
        let key = (ie.ie_type, ie.instance);
        let (clause, kind) = match ie.ie_type {
            // A Private Extension of any instance is no IE of a table of its own.
            255 => return None,
            _ if ie.instance > 0x0f => ("8.2.1", DeviationKind::SpareBits),
            known if ie_type_info(known).is_some() => {
                ("7.7.9", DeviationKind::Unexpected { ie: key })
            }
            _ => ("7.7.9", DeviationKind::UnknownType { ie: key }),
        };
        Some(Deviation::new(path, clause, kind))
    }

    #[inline]
    fn is_null(ie: &crate::ie::RawIe) -> bool {
        ie.octets.is_empty()
    }

    #[inline]
    fn frames(ie: &crate::ie::RawIe) -> bool {
        ie.frames()
    }

    #[inline]
    fn clear_length(ie: &mut crate::ie::RawIe) {
        ie.length = None;
    }

    fn name_of(key: (u16, u8), table: Option<&Table>) -> String {
        crate::tree_name(key, table)
    }

    #[inline]
    fn unframed_within(row: &Row, ie: &crate::ie::RawIe) -> Option<(u16, u8)> {
        unframed_within(row, &ie.octets)
    }
}
