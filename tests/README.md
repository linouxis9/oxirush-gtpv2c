# The tests

- `spec.rs`: the crate against the fixtures of TS 29.274 V19.6.0 in `spec/`, which
  `spec/extract.py` writes from the document; `spec/errata.tsv` says what the crate reads
  otherwise, and why. `spec/draft.py` drafts the registry from the fixtures.
- `captures.rs`: the GTPv2-C messages of `captures/messages.tsv`, which
  `captures/extract.py` writes from a capture of the loopback of a lab, each byte-exact
  through its tree, and checked. The messages are those of one lab run of EPS attaches,
  detaches, handovers and probes of S1 against Open5GS v2.8.0 (157f611a5), on 2026-11-15:
  the MME at
  127.0.0.2, the SGW-C at 127.0.0.3 and the PGW-C at 127.0.0.4, S11 and S5 on UDP port
  2123. The IMSIs are those of the lab's test PLMN 208/93.
- `first.rs`: a first message of each kind, written octet by octet.
- `vectors.rs`: the messages of the tests of go-gtp v0.8.12 (`vectors/go-gtp.tsv`, which
  `vectors/extract.py` writes from its module), each byte-exact through its tree, with
  what the check finds in them.
