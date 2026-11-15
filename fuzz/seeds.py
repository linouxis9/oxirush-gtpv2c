#!/usr/bin/env python3
"""The seeds of the fuzz targets: of each message type of the captures and of go-gtp's
vectors, the three longest messages, each in a file named by the digest of its octets.

    python3 fuzz/seeds.py       # from the directory of the crate
"""
import hashlib
import os
from collections import defaultdict

by_type = defaultdict(set)
for path, column in [('tests/captures/messages.tsv', 5), ('tests/vectors/go-gtp.tsv', 2)]:
    for line in open(path, encoding='utf-8'):
        if line.startswith('#'):
            continue
        octets = bytes.fromhex(line.rstrip('\n').split('\t')[column])
        by_type[octets[1]].add(octets)
count = 0
for message_type, messages in sorted(by_type.items()):
    for octets in sorted(messages, key=len, reverse=True)[:3]:
        name = hashlib.sha256(octets).hexdigest()[:16]
        with open(os.path.join('fuzz', 'seeds', name), 'wb') as out:
            out.write(octets)
        count += 1
print(f'{count} seeds of {len(by_type)} message types')
