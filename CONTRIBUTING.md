# Contributing to oxirush-gtpv2c

## Tests

```sh
cargo test --all-features
```

[`tests/README.md`](tests/README.md) says what each test checks. The
fixtures are written by scripts that sit beside them:

| Fixtures | Written by | From |
|----------|------------|------|
| `tests/spec/*.tsv` | `tests/spec/extract.py` | TS 29.274 V19.6.0 in .docx, with the reader of oxirush-utils (`tools/docx_tables.py`); `errata.tsv` says what the crate reads otherwise, and why |
| `tests/captures/messages.tsv` | `tests/captures/extract.py` | A capture of the GTPv2-C of a lab run of Open5GS v2.8.0 |
| `tests/vectors/go-gtp.tsv` | `tests/vectors/extract.py` | The tests of go-gtp v0.8.12 |

`tests/spec/draft.py` drafts the registry of the crate from the fixtures of
the specification, for a new release of it.

## Fuzzing

```sh
python3 fuzz/seeds.py
cargo +nightly fuzz run decode -- -max_total_time=60
```

`fuzz/` has three [cargo-fuzz](https://crates.io/crates/cargo-fuzz) targets:
`decode` reads any octets, `tree` reads the tree of a decoded message back
to the same octets and checks it as the message is, and `paths` selects and
sets any path in the tree of a decoded message.
`seeds.py` writes their seeds: the three longest messages of each type of the
captures and the vectors. CI runs each
for a minute at a push, an hour each day but Sunday, and four hours on
Sunday.

## Pull requests

A pull request comes with a test for what it changes. Sign off your commits
(`git commit -s`): by contributing you agree to the [Developer Certificate
of Origin](https://developercertificate.org/). [SECURITY.md](SECURITY.md)
says how to report a vulnerability.
