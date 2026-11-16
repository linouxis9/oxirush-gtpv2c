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
    let echo = message.body.as_echo_request().ok_or("another message")?;
    assert_eq!(echo.recovery.as_ref().map(|recovery| recovery.value), Some(5));

    // The SGW answers with its own Restart Counter, on the same sequence number
    let response = Message::path(message.header.sequence, EchoResponse::new(Recovery::new(9)));
    assert_eq!(hex::encode(response.encode()?), "40020009000001000300010009");
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

## Build a message, and check it

A message is a struct with a field for each row of its table. `new()` takes
the mandatory IEs, a `with_` method adds each other one, and a value is a
struct of the fields of its figure. `check()` compares a message with its
table of clause 7: each deviation has its path, its clause and the Cause with
which a receiver answers a request.

```rust
use std::net::Ipv4Addr;

use oxirush_gtpv2c::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // An MME asks its SGW for a session on S11: its F-TEID, the APN, and the default
    // bearer, EBI 5 of QCI 9 and ARP priority level 8
    let mme = FTeid::ipv4(InterfaceType::S11MmeGtpC, 1, Ipv4Addr::new(10, 0, 0, 1));
    let bearer = BearerContextToBeCreatedInCreateSessionRequest::new(
        Ebi::new(5),
        BearerLevelQualityOfService::new(9, 8),
    );
    let mut request =
        CreateSessionRequest::new(RatType::new(RatTypeValue::Eutran), mme, Apn::new("internet")?, [bearer])
            .with_imsi(Imsi::new("208930000000001")?)
            .with_pdn_type(PdnType::new(PdnTypeValue::Ipv4));
    assert!(Message::tunnel(0, 2, request.clone()).check().is_empty());

    // Without its mandatory RAT Type, the SGW answers "Mandatory IE missing", which
    // names the IE by its type and instance
    request.clear_rat_type();
    let found = Message::tunnel(0, 2, request).check();
    let missing = "the mandatory RAT Type is missing (TS 29.274 7.7.6), at /gtpv2c/RAT Type: \
                   the answer is \"Mandatory IE missing\"";
    assert_eq!(found.len(), 1);
    assert_eq!(found[0].to_string(), missing);
    assert_eq!(found[0].cause, Some(CauseValue::MandatoryIeMissing));
    assert_eq!(found[0].offending_ie(), Some((82, 0)));

    // The answer of the SGW, a rejection with its Cause alone (§6.1.1)
    let response = Message::tunnel(1, 2, CreateSessionResponse::rejected(CauseValue::MandatoryIeMissing));
    assert!(response.check().is_empty());
    assert_eq!(response.to_string(), "Create Session Response (TEID 0x1, sequence 2)");
    Ok(())
}
```

`check()` also finds an instance that no row has (§7.7.9), a repeated IE
(§7.7.10), a Length that is not that of the content (§7.7.7), the IEs that a
rejection may carry (§6.1.1), and the conditions that the message shows alone.
An IE that the table of its scope does not have, of another instance, with its
spare bits set, or that does not decode is kept as its octets in its place.

## Read and change the IEs of a message

`as_<message>()` and `as_<message>_mut()` give the message that a decoded body
is. Each row is a public field: an `Option` of a row there once, a `Vec` of a
list, a struct of the content of a grouped IE. A mandatory row also has an
accessor of its name, `cause()`, which a received message may lack: it gives
the value, or an error that names the row and its table. `message.cause()` and
`message.recovery()` read those IEs of any message. In place, `set_<row>()` gives a
row its value, `push_<row>()` adds one to a list and `clear_<row>()` removes
it; `with_<row>()` builds. Codes are named values, with `Other` for a number
that their table does not name.

```rust
use std::net::Ipv4Addr;

use oxirush_gtpv2c::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // A Create Session Response of the SGW: accepted, the SGW's F-TEID of S11 and the
    // bearer created, EBI 5, with its S1-U F-TEID
    let sgw = Ipv4Addr::new(10, 0, 0, 2);
    let s1u = FTeid::ipv4(InterfaceType::S1uSgwGtpU, 0x2001, sgw);
    let created = BearerContextCreatedInCreateSessionResponse::new(Ebi::new(5), Cause::new(CauseValue::RequestAccepted))
        .with_s1_u_sgw_f_teid(s1u);
    let response = CreateSessionResponse::new(Cause::new(CauseValue::RequestAccepted), [created])
        .with_sender_f_teid_for_control_plane(FTeid::ipv4(InterfaceType::S11S4SgwGtpC, 0x1001, sgw));
    let octets = Message::tunnel(1, 2, response).encode()?;

    // The MME reads the IEs that it needs: the TEID of S1-U of each bearer
    let mut message = Message::decode(&octets)?;
    assert_eq!(message.cause(), Some(CauseValue::RequestAccepted));
    let response = message.body.as_create_session_response().ok_or("another message")?;
    let bearer = &response.bearer_contexts_created()?[0];
    let teid = bearer.s1_u_sgw_f_teid.as_ref().map(|f_teid| (f_teid.interface_type, f_teid.teid));
    assert_eq!(teid, Some((InterfaceType::S1uSgwGtpU, 0x2001)));

    // A test changes it in place: another Cause, a second bearer, no F-TEID of S11
    let response = message.body.as_create_session_response_mut().ok_or("another message")?;
    response.set_cause(Cause::new(CauseValue::RequestAcceptedPartially));
    response.push_bearer_contexts_created(BearerContextCreatedInCreateSessionResponse::new(
        Ebi::new(6),
        Cause::new(CauseValue::NoResourcesAvailable),
    ));
    response.clear_sender_f_teid_for_control_plane();
    let found: Vec<String> = message.check().iter().map(ToString::to_string).collect();
    assert_eq!(found, Vec::<String>::new());
    assert_eq!(Message::decode(&message.encode()?)?, message);

    // Without its Cause, the response is refused by name
    let response = message.body.as_create_session_response_mut().ok_or("another message")?;
    response.clear_cause();
    let missing = response.cause().unwrap_err().to_string();
    assert_eq!(missing, "the mandatory Cause of the Create Session Response (Table 7.2.2-1) is missing");
    Ok(())
}
```

## Edit a message as JSON

With the `tree` feature, a message is a tree: its name, its header and its IEs
by the names of its table, in the order of the wire. `paths::set()` edits it,
and `from_tree()` writes the message again.

```rust
use oxirush_gtpv2c::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let echo = Message::path(1, EchoRequest::new(Recovery::new(5)));
    let mut tree = echo.to_tree();
    let json = r#"{"message":"Echo Request","header":{"Sequence Number":1},"ies":{"Recovery":5}}"#;
    assert_eq!(tree.to_string(), json);

    // Another Restart Counter, set by its path
    paths::set(&mut tree, "/gtpv2c/Recovery", 6)?;
    let edited = Message::from_tree(&tree)?;
    assert_eq!(hex::encode(edited.encode()?), "40010009000001000300010006");
    Ok(())
}
```

## Carry an MM Context on N26

An AMF sends an MME the UE's EPS security context in the MM Context of a
Forward Relocation Request. `Debug` prints none of its keys.

```rust
use std::net::Ipv4Addr;

use oxirush_gtpv2c::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // The context: an EPS security context of KSIASME 1, 128-EIA2 and EEA0, the NAS
    // COUNTs, KASME, and NH with its NCC
    let context = MmContextEpsSecurityContextQuadrupletsAndQuintuplets {
        security_mode: SecurityMode::EpsSecurityContextAndQuadruplets,
        ksi_asme: 1,
        used_nas_integrity: NasIntegrity::Eia2,
        used_nas_cipher: NasCipher::None,
        nas_downlink_count: 5,
        nas_uplink_count: 3,
        kasme: Key([0x11; 32]),
        nh: Some(NextHop { nh: Key([0x22; 32]), ncc: 2, ..Default::default() }),
        subscribed_ue_ambr: Some((100_000, 200_000)),
        ue_network_capability: Octets::copied(&[0xf0, 0xf0, 0xc0, 0xc0]),
        ..Default::default()
    };
    let amf = FTeid::ipv4(InterfaceType::N26AmfGtpC, 0x1001, Ipv4Addr::new(10, 0, 0, 1));
    let request = ForwardRelocationRequest::new(amf).with_mme_sgsn_amf_ue_mm_context_eps(context);
    let octets = Message::tunnel(0, 7, request).encode()?;

    // The MME decodes the request and reads the context
    let message = Message::decode(&octets)?;
    assert!(message.check().is_empty());
    let request = message.body.as_forward_relocation_request().ok_or("another message")?;
    let context = request.mme_sgsn_amf_ue_mm_context_eps.as_ref().ok_or("no MM Context")?;
    let ncc = context.nh.map(|next| next.ncc);
    assert_eq!((context.ksi_asme, context.nas_downlink_count, ncc), (1, 5, Some(2)));
    assert_eq!(context.used_nas_integrity, NasIntegrity::Eia2);

    // The keys are in the octets, and not in `Debug`
    assert_eq!(format!("{:?}", context.kasme), "Key(32 octets)");
    assert!(!format!("{message:?}").contains("17, 17"));
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
