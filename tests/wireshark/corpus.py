#!/usr/bin/env python3
"""The corpus of the test `wireshark` and of compare.py: a message a line, its name and its
octets in hexadecimal, each message once: go-gtp's vectors, the messages of the lab
captures, and the files of each DIR.

    python3 tests/wireshark/corpus.py OUT.tsv [DIR...]

A DIR is a directory of messages, one a file, as fuzz/seeds and the corpus of a fuzz
target are.
"""
import os
import sys

if len(sys.argv) < 2:
    sys.exit(__doc__)
HERE = os.path.dirname(os.path.abspath(__file__))
out, directories = sys.argv[1], sys.argv[2:]
seen, rows = set(), []


def add(name, octets):
    if octets and octets not in seen:
        seen.add(octets)
        rows.append((name, octets.hex()))


sources = [
    # file, description, message
    (os.path.join(HERE, '..', 'vectors', 'go-gtp.tsv'), lambda f: (f'go-gtp/{f[0]}/{f[1]}', f[2])),
    # run, number, source, destination, interface, message
    (os.path.join(HERE, '..', 'captures', 'messages.tsv'), lambda f: (f'captures/{f[0]}/{f[1]}', f[5])),
]
for path, row in sources:
    for line in open(path, encoding='utf-8'):
        if line.startswith('#') or not line.strip():
            continue
        name, octets = row(line.rstrip('\n').split('\t'))
        add(name, bytes.fromhex(octets))
for directory in directories:
    label = os.path.normpath(directory)
    for name in sorted(os.listdir(directory)):
        with open(os.path.join(directory, name), 'rb') as f:
            add(f'{label}/{name}', f.read())
with open(out, 'w', encoding='utf-8') as f:
    for name, octets in rows:
        f.write(f'{name}\t{octets}\n')
print(len(rows), 'messages')
