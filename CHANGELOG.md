# Changelog

All notable changes to `oxirush-gtpv2c` are recorded here.

## 0.1.0 - unreleased

The first version, on GitHub and not on crates.io. It implements TS 29.274
V19.6.0 (2026-03):

- Decodes and encodes every message type that has a table of clause 7, byte for
  byte, each IE by its type and instance. An IE that its table does not have is
  kept as its octets in its place.
- `check()` validates a message against the tables of clause 7: mandatory IEs,
  instances of no row, repeated IEs, lengths, the IEs of a rejection, and the
  conditions that the message shows alone. Each deviation has its path, its
  clause and the Cause of the answer.
- With `tree`, a message converts to a tree that prints as JSON, in the order of
  the wire, and reads and edits by path.
- The MM Context of N26 (EPS security context) is laid out field by field, and
  `Debug` prints none of its keys.
