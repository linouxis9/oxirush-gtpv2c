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

//! The macros of the crate: those of oxirush-utils, of the modules of GTPv2-C.

/// The IE types of Table 8.1-1, one line each: its number, its name, its form (`fixed`,
/// `variable`, `extendable`, `grouped`), the number of its fixed octets, its clause and its
/// type of the crate, or each table of a grouped IE.
macro_rules! gtpv2c_ies {
    ($($ies:tt)*) => {
        oxirush_utils::ies! { {proto: crate::Gtpv2c, table: table, ie: ie} $($ies)* }
    };
}

/// A value of a layout of clause 8: fields that are always there, then an octet of flags
/// and fields each present when its condition over the flags holds, then the octets that
/// a later release adds, which the value keeps (§8.2.1, "Extendable").
macro_rules! gtpv2c_layout {
    ($($value:tt)*) => {
        oxirush_utils::layout! { [crate::ie::Codec, crate::check::Rules, crate::ie::IeType, crate::check::Deviation, crate::Error, crate::fields::field_error] $($value)* }
    };
}

/// A value of octets of bits: flags of one bit and fields of a few, each at its octet
/// (from 0), its lowest bit (from 1) and its width, in at most `N` octets of which the
/// first `MIN` are always there.
#[allow(unused_macros)]
macro_rules! gtpv2c_bits {
    ($($value:tt)*) => {
        oxirush_utils::bits! { [crate::ie::Codec, crate::check::Rules, crate::ie::IeType, crate::check::Deviation, crate::Error, crate::fields::field_error] $($value)* }
    };
}

/// The messages of Table 6.1-1 that have a table of clause 7, one each: their type and
/// the struct of their table.
macro_rules! gtpv2c_messages {
    ($($messages:tt)*) => {
        oxirush_utils::messages! {
            {proto: crate::Gtpv2c, table: table, ie: ie, check: check, paths: paths, message: message, registry: registry, error: crate::Error}
            $($messages)*
        }
    };
}

/// A message or the content of a grouped IE in one place, from its table of clause 7: a
/// struct with a field for each row, `new()` with the mandatory IEs, a `with_` builder for
/// each row, the codec, the check and the tree. A row is `field: "Name" P TYPE INSTANCE
/// []`, where `P` is `M`, `C`, `CO` or `O`, followed by `*` for a list.
macro_rules! gtpv2c_table {
    ($($table:tt)*) => {
        oxirush_utils::table! {
            {proto: crate::Gtpv2c, table: table, ie: ie, check: check, paths: paths, error: crate::Error, ctors: gtpv2c_ctors}
            $($table)*
        }
    };
}

/// The constructors by interface, which GTPv2-C does not have: its tables have no columns
/// of interfaces, and the table macro calls it for none.
#[allow(unused_macros)]
macro_rules! gtpv2c_ctors {
    (@ctors $fam:tt [$($by:ident)*] $ty:ident $rows:tt) => {};
}

#[allow(unused_imports)]
pub(crate) use gtpv2c_bits;
#[allow(unused_imports)]
pub(crate) use gtpv2c_ctors;
pub(crate) use {gtpv2c_ies, gtpv2c_layout, gtpv2c_messages, gtpv2c_table};
