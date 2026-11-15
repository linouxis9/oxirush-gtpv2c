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

//! The registry: every IE type of Table 8.1-1, one line each, and the tables of clause 7,
//! one row each, by IE type and instance, which the macros expand into the types of the
//! crate. A grouped IE has a type of the crate for each place, of its table there.

use crate::enums::CauseValue;
use crate::macros::{gtpv2c_ies, gtpv2c_table};
use crate::mm_context::*;
use crate::tft::{EpsBearerLevelTrafficFlowTemplate, Tad};
use crate::uli::Uli;
use crate::values::*;
use crate::values_hand::*;

pub use oxirush_utils::tables::Form;

// The conditions of the rows that a scope alone shows (tests/spec/conditions.tsv).

/// Whether the Cause accepts the request: "Request accepted" and the other acceptances
/// (Table 8.4-1).
fn accepted(cause: &Option<Cause>) -> bool {
    cause
        .as_ref()
        .is_some_and(|cause| (16..=63).contains(&cause.value.value()))
}

/// Whether the Cause is `value`.
fn cause_is(cause: &Option<Cause>, value: CauseValue) -> bool {
    cause.as_ref().is_some_and(|cause| cause.value == value)
}

/// An IE type as Table 8.1-1 has it, of the same fields with every feature.
///
/// ```
/// use oxirush_gtpv2c::{Form, ie_type_info};
///
/// let recovery = ie_type_info(3).unwrap();
/// assert_eq!((recovery.name, recovery.form, recovery.clause), ("Recovery (Restart Counter)", Form::Variable, "8.5"));
/// ```
pub type IeTypeInfo = oxirush_utils::tables::IeTypeInfo<crate::Gtpv2c>;

/// The IE type of `ie_type`.
pub fn ie_type_info(ie_type: u16) -> Option<&'static IeTypeInfo> {
    IE_TYPES
        .binary_search_by_key(&ie_type, |info| info.ie_type)
        .ok()
        .map(|at| &IE_TYPES[at])
}

include!("registry_tables.rs");
