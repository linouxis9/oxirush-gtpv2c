# oxirush-gtpv2c

[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)

Decode, build, check and encode GTPv2-C, the control protocol between the MME,
the SGW and the PGW of 4G cores and of N26 between an MME and an AMF, in Rust
(3GPP TS 29.274 V19.6.0). It covers every message and IE of S11, S5/S8, S3,
S10, S16 and N26, byte for byte, and converts them to and from JSON. The crate
is `no_std`. It is a first version, 0.1.0, on GitHub and not yet on crates.io.

```rust
use oxirush_gtpv2c::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // An MME asks its SGW whether it is alive: an Echo Request with its Restart Counter
    let octets = Message::path(1, EchoRequest::new(Recovery::new(5))).encode()?;
    assert_eq!(hex::encode(&octets), "40010009000001000300010005");

    // The SGW decodes it and reads its IEs as typed fields
    let message = Message::decode(&octets)?;
    assert_eq!(message.to_string(), "Echo Request (sequence 1)");
    let Body::EchoRequest(echo) = &message.body else { unreachable!() };
    assert_eq!(echo.recovery.as_ref().map(|recovery| recovery.value), Some(5));

    // Its tree, which reads back to the same octets
    let tree = message.to_tree();
    let json = r#"{"message":"Echo Request","header":{"Sequence Number":1},"ies":{"Recovery":5}}"#;
    assert_eq!(tree.to_string(), json);
    assert_eq!(Message::from_tree(&tree)?.encode()?, octets);
    Ok(())
}
```

## Installation

```toml
[dependencies]
oxirush-gtpv2c = { git = "https://github.com/linouxis9/oxirush-gtpv2c", features = ["tree"] }
hex = "0.4" # for the examples of this page
```

| Feature | Adds | Needs |
|---------|------|-------|
| `tree` | Messages and IEs as JSON trees, with reads and edits by path | |

The crate is `no_std` with `alloc`, and is written with the fields and tables
of oxirush-utils 0.3. The minimum supported Rust version is 1.88.

## Check a message, and answer a faulty one

`check()` compares a message with the table of clause 7 that its type has: an
IE by its type and instance, each with its presence. Each deviation has its
path, its clause and the Cause with which a receiver answers a request.

```rust
use oxirush_gtpv2c::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // A Create Session Request without its mandatory RAT Type
    let mut tree = tree!({
        "message": "Create Session Request",
        "header": {"TEID": 0, "Sequence Number": 2},
        "ies": {
            "IMSI": "208930000000001",
            "Sender F-TEID for Control Plane": {
                "V4": true, "Interface Type": 10, "TEID / GRE Key": 1, "IPv4 address": "10.0.0.1"
            },
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
    let missing = "the mandatory RAT Type is missing (TS 29.274 7.7.6), at /gtpv2c/RAT Type: \
                   the answer is \"Mandatory IE missing\"";
    assert_eq!(found[0].to_string(), missing);

    // The Cause of the answer names the IE by its type and instance
    assert_eq!(found[0].cause, Some(CauseValue::MandatoryIeMissing));
    assert_eq!(found[0].offending_ie(), Some((82, 0)));

    // paths::set() adds it, as E-UTRAN, and the request is then whole
    paths::set(&mut tree, "/gtpv2c/RAT Type", 6)?;
    assert!(Message::from_tree(&tree)?.check().is_empty());
    Ok(())
}
```

`check()` also finds an instance that no row has (§7.7.9), a repeated IE
(§7.7.10), a Length that is not that of the content (§7.7.7), the IEs that a
rejection may carry (§6.1.1), and the conditions that the message shows alone.
An IE that the table of its scope does not have, of another instance, with its
spare bits set, or that does not decode is kept as its octets in its place.

## Carry an MM Context on N26

An AMF sends an MME the UE's EPS security context in the MM Context of a
Forward Relocation Request. The tree has the keys, and `Debug` prints none of
them.

```rust
use oxirush_gtpv2c::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let kasme = "000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f";
    let nh = "a0a1a2a3a4a5a6a7a8a9aaabacadaeafb0b1b2b3b4b5b6b7b8b9babbbcbdbebf";
    let tree = tree!({
        "message": "Forward Relocation Request",
        "header": {"TEID": 0, "Sequence Number": 7},
        "ies": {
            "IMSI": "208930000000001",
            "Sender's F-TEID for Control Plane": {
                "V4": true, "Interface Type": 12, "TEID / GRE Key": 4097,
                "IPv4 address": "10.0.0.1"
            },
            "MME/SGSN/AMF UE MM Context (EPS Security Context, Quadruplets and Quintuplets)": {
                "Security Mode": 4, "KSIASME": 1,
                "Used NAS integrity protection algorithm": 2, "Used NAS Cipher": 0,
                "NAS Downlink Count": 5, "NAS Uplink Count": 3,
                "KASME": kasme, "NH": nh, "NCC": 2,
                "Uplink Subscribed UE AMBR": 100000, "Downlink Subscribed UE AMBR": 200000,
                "UE Network Capability": "f0f0c0c0", "MS Network Capability": "",
                "Mobile Equipment Identity (MEI)": "",
                "Access restriction data": {"NBNA": true},
                "Voice Domain Preference and UE's Usage Setting": "",
                "UE Radio Capability for Paging information": "",
                "Extended Access Restriction Data": "",
                "UE additional security capability": "",
                "UE NR security capability": "f0f0"
            }
        }
    });

    // The MME decodes the request, and reads the context as typed fields
    let octets = Message::from_tree(&tree)?.encode()?;
    let message = Message::decode(&octets)?;
    assert!(message.check().is_empty());
    let Body::ForwardRelocationRequest(request) = &message.body else { unreachable!() };
    let context = request.mme_sgsn_amf_ue_mm_context_eps.as_ref().ok_or("no MM Context")?;
    let ncc = context.nh.as_ref().map(|(_, ncc)| *ncc);
    assert_eq!((context.ksi_asme, context.nas_downlink_count, ncc), (1, 5, Some(2)));

    // The keys are in the tree and the octets, and not in `Debug`
    assert!(!format!("{message:?}").contains("a0a1a2"));
    Ok(())
}
```

## Limits

- The checks have been run against the captures of one core alone: the
  GTPv2-C of an EPS lab of Open5GS v2.8.0 (S11 and S5), and the vectors of
  go-gtp's tests.
- N26 has been checked by reading: Wireshark reads the octets that the crate
  writes, and no AMF or MME has exchanged them with it.
- It is a codec: it has no node or procedure layer, no retransmission, timers,
  sequence numbers or TEID allocation.
- `check()` does not judge the conditions of a C row that depend on more than
  the message, nor the content of a value that the crate keeps as its octets.
- Nothing has been measured yet: it has no benchmarks.

## Tests and fuzzing

```sh
cargo test --all-features
cargo run --features tree --example inspect -- 40010009000001000300010005
```

Each example of this page checks what it gives with `assert_eq!`, and runs as
a documentation test. The tests compare the crate with the tables and figures
of TS 29.274, read back the GTPv2-C messages of an Open5GS lab run and the
vectors of go-gtp byte for byte, and check an MM Context of N26 field by field.
[`tests/README.md`](tests/README.md) says what each test checks.

[CONTRIBUTING.md](CONTRIBUTING.md) has the fixtures, the fuzz targets and how
to contribute. [SECURITY.md](SECURITY.md) says how to report a vulnerability.

## The OxiRush family

- [oxirush-utils](https://github.com/linouxis9/oxirush-utils): the 3GPP identities, the JSON tree with its paths, and the fields and tables that the codecs are laid out on.
- [oxirush-aper](https://github.com/linouxis9/oxirush-aper): aligned PER, the runtime of the NGAP and S1AP codecs.
- [oxirush-nas](https://github.com/linouxis9/oxirush-nas): the NAS messages of 5GS and EPS.
- [oxirush-security](https://github.com/linouxis9/oxirush-security): NAS security, AKA and the derivation of keys.
- [oxirush-ngap](https://github.com/linouxis9/oxirush-ngap): NGAP, between a gNB and an AMF.
- [oxirush-s1ap](https://github.com/linouxis9/oxirush-s1ap): S1AP, between an eNB and an MME.
- [oxirush-gtp-u](https://github.com/linouxis9/oxirush-gtp-u): GTP-U tunnels, with a user-plane test peer.
- [oxirush-gtpv2c](https://github.com/linouxis9/oxirush-gtpv2c): GTPv2-C, between the MME, the SGW and the PGW, and on N26.
- [oxirush-pfcp](https://github.com/linouxis9/oxirush-pfcp): PFCP, between the control plane and the user plane.
- [oxirush-sctp](https://github.com/linouxis9/oxirush-sctp): SCTP sockets, which carry NGAP and S1AP.

## License

Copyright 2026 Valentin D'Emmanuele. Licensed under the Apache License,
Version 2.0: see [LICENSE](LICENSE).
