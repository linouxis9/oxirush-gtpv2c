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

//! The fields of the values of clause 8, read in network order: those of oxirush-utils,
//! and how a value writes them ([`put`]).

use alloc::vec::Vec;

use crate::Error;

#[allow(unused_imports)]
pub(crate) use oxirush_utils::fields::*;
#[cfg(feature = "spec")]
#[doc(hidden)]
pub use oxirush_utils::fields::{Bits, Layout, Member, NarrowCell};
pub use oxirush_utils::fields::{Rest, SecretRest, U24, U40};

/// Write `field`, named `name`, of a value of the IE `ie`: refused where its length, its
/// number or its type do not fit the bits that say them.
#[allow(dead_code)]
pub(crate) fn put<F: Field>(
    field: &F,
    ie: &'static str,
    name: &'static str,
    out: &mut Vec<u8>,
) -> Result<(), Error> {
    oxirush_utils::fields::put(field, ie, name, out).map_err(field_error)
}

/// The error of a value that does not encode.
pub(crate) fn field_error(error: FieldError) -> Error {
    match error {
        FieldError::Length { ie, field } => Error::FieldLength { ie, field },
        FieldError::Inconsistent { ie, field, clause } => Error::Inconsistent { ie, field, clause },
    }
}
