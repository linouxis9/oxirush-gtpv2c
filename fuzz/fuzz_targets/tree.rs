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

//! The tree of a decoded message reads back to a message that encodes to the same octets,
//! and is checked as the message is.

#![no_main]

use libfuzzer_sys::fuzz_target;
use oxirush_gtpv2c::Message;

fuzz_target!(|octets: &[u8]| {
    let Ok(message) = Message::decode(octets) else {
        return;
    };
    let tree = message.to_tree();
    let read = Message::from_tree(&tree).unwrap_or_else(|error| panic!("{error}: {tree}"));
    // The same deviations, which a scope read from a tree reports in the order of its
    // kept IEs
    let sorted = |message: &Message| {
        let mut found: Vec<String> = message.check().iter().map(|d| format!("{d:?}")).collect();
        found.sort();
        found
    };
    assert_eq!(sorted(&read), sorted(&message), "{tree}");
    let again = read.encode().expect("a message read from a tree encodes");
    // The octets of a row whose IEs are apart on the wire come together in a tree
    if again != octets {
        let tree_again = Message::decode(&again)
            .expect("its octets decode")
            .to_tree();
        assert_eq!(tree_again, tree, "the tree of the octets of a tree");
    }
});
