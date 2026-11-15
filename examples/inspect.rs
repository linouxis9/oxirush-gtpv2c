//! The tree of a message and what `check()` finds in it, from its octets in hexadecimal:
//!
//! ```text
//! cargo run --features tree --example inspect -- 40010009000001000300010005
//! ```

fn main() {
    let Some(octets) = std::env::args()
        .nth(1)
        .and_then(|text| hex::decode(text).ok())
    else {
        eprintln!("usage: inspect HEX");
        std::process::exit(2);
    };
    match oxirush_gtpv2c::Message::decode(&octets) {
        Ok(message) => {
            println!("{}", message.to_tree());
            for deviation in message.check() {
                println!("{deviation}");
            }
        }
        Err(error) => {
            eprintln!("{error}");
            std::process::exit(1);
        }
    }
}
