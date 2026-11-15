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

//! The tables of clause 7: for a message or a grouped IE, the IEs that it can have, each
//! by its IE type and instance, with its presence and how many times it may be there;
//! and what a typed scope does with them, which oxirush-utils writes for every table.

use alloc::vec::Vec;

use crate::Error;
use crate::ie::{Codec, IeType};

pub use oxirush_utils::tables::Repeat;
pub(crate) use oxirush_utils::tables::{
    FieldValues, MAX_ROWS, decode_scope, decode_scope_into, decode_scope_sized, encode_scope,
};

/// One row of a table of clause 7: an IE that the scope can have, by its IE type and
/// instance, a field of the scope.
pub type Row = oxirush_utils::tables::Row<crate::Gtpv2c>;
/// A row of a table, as the field of a scope has it.
pub type Role = oxirush_utils::tables::Role<crate::Gtpv2c>;
/// A table of clause 7: of a message, or of the content of a grouped IE in one place.
pub type Table = oxirush_utils::tables::Table<crate::Gtpv2c>;
/// What a row is in a message, as [`Row::on`] gives it.
pub type On = oxirush_utils::tables::On<crate::Gtpv2c>;

/// The presence of an IE in its table (§6.1.1).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Presence {
    /// M: the sender includes it, and the receiver diagnoses "Mandatory IE missing"
    /// without it.
    Mandatory,
    /// C: the sender includes it when the condition of its row holds.
    Conditional,
    /// CO: an up-to-date sender includes it when the condition of its row holds; the
    /// receiver treats it as optional.
    ConditionalOptional,
    /// O: the sender may include it.
    Optional,
}

impl oxirush_utils::tables::Presence for Presence {
    const OPTIONAL: Self = Self::Optional;

    #[inline]
    fn is_mandatory(self) -> bool {
        self == Self::Mandatory
    }

    #[inline]
    fn rank(self) -> u8 {
        match self {
            Self::Mandatory => 0,
            Self::Conditional => 1,
            // The receiver handles a CO IE as an optional one (§6.1.1).
            Self::ConditionalOptional | Self::Optional => 2,
        }
    }
}

/// The interfaces of a row. The tables of TS 29.274 give a row's interfaces in the words
/// of its condition, not in columns: a row is on each interface, and a table's rows are
/// written with none.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Interfaces(pub(crate) u8);

impl Interfaces {
    /// Every interface.
    pub const ALL: Self = Self(0);
}

impl oxirush_utils::tables::Interfaces for Interfaces {
    const NONE: Self = Self(0);

    #[inline]
    fn has(self, _other: Self) -> bool {
        true
    }

    #[inline]
    fn covers(self, _other: Self) -> bool {
        true
    }

    #[inline]
    fn union(self, other: Self) -> Self {
        Self(self.0 | other.0)
    }

    #[inline]
    fn each(self) -> impl Iterator<Item = Self> {
        core::iter::once(self)
    }
}

/// The table of the content of a value: of a grouped IE, the table of its type of the
/// crate, which is one place's; none for another value. The registry writes it for each
/// type.
pub(crate) trait Content {
    const TABLE: Option<&'static Table> = None;
}

/// How many IEs `roles` allow together, `usize::MAX` for any number: what the field of a
/// row holds, which the table macro asserts.
#[allow(dead_code)]
pub(crate) const fn capacity(roles: &[Role]) -> usize {
    oxirush_utils::tables::capacity(roles)
}

/// What the table macro writes for a message or a grouped IE, which the code of every
/// scope takes, of GTPv2-C.
pub(crate) trait Scope: oxirush_utils::tables::Scope<P = crate::Gtpv2c> {}

impl<S: oxirush_utils::tables::Scope<P = crate::Gtpv2c>> Scope for S {}

/// Write the `nth` value of a field as its IE, of the key of its row: `false` when it has
/// no such value.
#[inline]
pub(crate) fn emit_field<T: IeType + Codec, F: FieldValues<T>>(
    key: (u16, u8),
    field: &F,
    nth: usize,
    out: &mut Vec<u8>,
) -> Result<bool, Error> {
    match field.values().get(nth) {
        Some(value) => crate::ie::encode_value(key, value, out).map(|()| true),
        None => Ok(false),
    }
}
