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
- `n26.rs`: a Forward Relocation Request of N26 with an MM Context of an EPS security
  context, written as a tree, byte-exact through it, its keys out of `Debug`. Wireshark
  4.6.4 reads its octets field by field as the crate writes them (`GTPV2C_PRINT=1` prints
  them).
- `differential.rs`: what comparing the crate with Wireshark found, each a value on which
  the two read a field otherwise, and what the clause says of it.

## Wireshark beside the crate

`wireshark.rs` writes what the crate reads of each message of a corpus: whether it
decodes, its header, what check() tells of it, and its IEs at every level of the grouped
IEs with their offsets, their type, Length and instance, and the tree of each value.
`wireshark/compare.py` has tshark 4.6.4 dissect the same messages, as datagrams to port
2123, and compares its PDML with that: the header, the IEs at each offset, and each field
of an IE that `wireshark/mapping.tsv` pairs with a member of the crate's tree. A message is
`agree`, `differs`, `both-refuse`, `ws-malformed` or `crate-refuses`, and each difference
is a row of `wireshark/known.tsv`, with its clause and whether it is Wireshark's, the
crate's by its clause, both's, a field that Wireshark does not dissect, or a value that the
crate keeps as its octets; one without a row fails the run, and `coverage.tsv` says how
each field was compared or why not. The workflow `wireshark` runs it each week on go-gtp's
vectors, the messages of the captures, the seeds of the fuzz targets and the corpora of
the targets decode and tree:

```sh
python3 tests/wireshark/corpus.py /tmp/corpus.tsv fuzz/seeds fuzz/corpus/decode
GTPV2C_WIRESHARK_CORPUS=/tmp/corpus.tsv GTPV2C_WIRESHARK_CRATE=/tmp/crate.jsonl \
    cargo test --release --all-features --test wireshark -- --ignored
python3 -I tests/wireshark/compare.py --corpus /tmp/corpus.tsv --crate /tmp/crate.jsonl \
    --mapping tests/wireshark/mapping.tsv --known tests/wireshark/known.tsv --output /tmp/wireshark
```

On 31223 messages here, 264434 fields compared, the comparison found three readings of the
crate to fix, each with its test in `differential.rs`: the RAC of an RAI is octet c+5
alone (§8.21.3), an extendable IE shorter than its fixed octets is an error (§7.7.7), and a
Cause is of 2 or 6 octets (§8.4). The other rows are Wireshark 4.6.4's (fixed fields read
past the Length of the IE, the spare bits of an ECI shown in it, a label shown up to a zero
octet), both's, the IEs of two grouped IEs that Wireshark does not dissect, the values
that the crate keeps as octets, and one of the crate that is held: an IE whose spare bits
of octet 4 are set is kept out of its row (§8.2.1), where check() tells its spare bits, or
that it is unexpected for an instance that its table does not have (§7.7.9).
