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

//! A path of any text, selected and set in the tree of a decoded message: an error or a
//! tree, never a panic; a set tree reads as a message or is refused.

#![no_main]

use libfuzzer_sys::fuzz_target;
use oxirush_gtpv2c::{Message, paths, tree};

fuzz_target!(|input: (&[u8], &str, u8)| {
    let (octets, path, value) = input;
    let Ok(message) = Message::decode(octets) else {
        return;
    };
    let mut tree = message.to_tree();
    let _ = paths::select(&tree, path);
    let _ = paths::row_at(&tree, path);
    if paths::set(&mut tree, path, tree::Value::from(u64::from(value))).is_ok() {
        let _ = Message::from_tree(&tree).map(|message| message.encode());
    }
});
