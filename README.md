# oxirush-gtpv2c

A Rust implementation of GTPv2-C (3GPP TS 29.274), the control protocol between the MME,
the SGW and the PGW of 4G core networks, and of the N26 interface between the MME and the
AMF. It decodes, builds and checks every message and IE of S11, S5/S8, S3, S10, S16 and
N26, keyed by IE type and instance as the tables of the specification are, and converts
them to and from JSON. Decoding and encoding are byte-exact. `no_std`.

```rust
use oxirush_gtpv2c::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // An MME asks its SGW whether it is alive: an Echo Request of its Restart Counter
    let octets = Message::path(1, EchoRequest::new(Recovery::new(5))).encode()?;
    assert_eq!(hex::encode(&octets), "40010009000001000300010005");

    // The SGW decodes it and reads its IEs as typed fields
    let message = Message::decode(&octets)?;
    assert_eq!(message.to_string(), "Echo Request (sequence 1)");
    let Body::EchoRequest(echo) = &message.body else { return Err("another message".into()) };
    assert_eq!(echo.recovery.as_ref().map(|recovery| recovery.value), Some(5));

    // Its tree, which reads back to the same octets
    let tree = message.to_tree();
    assert_eq!(tree.to_string(), r#"{"message":"Echo Request","header":{"Sequence Number":1},"ies":{"Recovery":5}}"#);
    assert_eq!(Message::from_tree(&tree)?.encode()?, octets);

    // A Create Session Request written as a tree, without its mandatory RAT Type:
    // check() says which IE is missing, where, and the Cause of the answer of the SGW,
    // which names the IE by its type and instance
    let mut tree = tree!({
        "message": "Create Session Request",
        "header": {"TEID": 0, "Sequence Number": 2},
        "ies": {
            "IMSI": "208930000000001",
            "Sender F-TEID for Control Plane": {"V4": true, "Interface Type": 10, "TEID / GRE Key": 1, "IPv4 address": "10.0.0.1"},
            "Access Point Name (APN)": "internet",
            "Bearer Contexts to be created": [{
                "EPS Bearer ID": 5,
                "Bearer Level QoS": {
                    "PL": 8, "Label (QCI)": 9,
                    "Maximum bit rate for uplink": 0, "Maximum bit rate for downlink": 0,
                    "Guaranteed bit rate for uplink": 0, "Guaranteed bit rate for downlink": 0
                }
            }]
        }
    });
    let request = Message::from_tree(&tree)?;
    let found = request.check();
    assert_eq!(found.len(), 1);
    assert_eq!(found[0].to_string(), "the mandatory RAT Type is missing (TS 29.274 7.7.6), at /gtpv2c/RAT Type: the answer is \"Mandatory IE missing\"");
    assert_eq!(found[0].offending_ie(), Some((82, 0)));

    // paths::set() adds it, as E-UTRAN
    paths::set(&mut tree, "/gtpv2c/RAT Type", 6)?;
    assert!(Message::from_tree(&tree)?.check().is_empty());
    Ok(())
}
```

Each example shows what it gives with `assert_eq!`, and runs as a test of the crate.

## What the crate has

- **The tables of clause 7**, one row per IE by its type and its instance, for each
  message and each grouped IE in each place (Bearer Context has a table per message):
  158 tables, 83 message types.
- **The IE types of Table 8.1-1**: the values that S11 and S5/S8 use most are laid out
  field by field (F-TEID, Bearer QoS, AMBR, EBI, APN, IMSI, MSISDN, MEI, Cause, Serving
  Network, PDN Type, ...); the others are kept as their octets, and the octets of an MM
  Context, which hold keys, are not printed by `Debug`.
- **`check()`**: the header (§5.5.1), the IEs that each table has, by type and instance:
  a mandatory IE missing (§7.7.6), an instance of no row (§7.7.9), a repeated IE (§7.7.10),
  a Length that is not the length of the content (§7.7.7), the IEs that a rejection may
  carry (§6.1.1), with the Cause of the answer of an initial message.
- **Byte-exact IEs**: an IE that the table of its scope does not have, of an instance that
  it does not have, with its spare bits set, or that does not decode is kept as its octets
  in its place, and a tree keeps the order of the wire.

## Limits

The conditions of the C rows, which are in words, are not checked, nor the content of the
values that the crate keeps as octets, nor the procedures of the specification
(retransmission, timers).

## License

Apache-2.0.
