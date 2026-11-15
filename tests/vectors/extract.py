#!/usr/bin/env python3
"""The messages of the tests of go-gtp's gtpv2/message, as tests/vectors/go-gtp.tsv has
them: the octets of each `Serialized` of a test case, by its file and its description.

    python3 tests/vectors/extract.py GO_GTP_DIR > tests/vectors/go-gtp.tsv

GO_GTP_DIR is the module of go-gtp v0.8.12, as the Go module cache has it.
"""
import os
import re
import sys

root = os.path.join(sys.argv[1], 'gtpv2', 'message')
CASE = re.compile(r'Description:\s*"([^"]*)".*?Serialized:\s*\[\]byte\{(.*?)\n\s*\},', re.S)
print('# file\tdescription\tmessage')
for name in sorted(os.listdir(root)):
    if not name.endswith('_test.go'):
        continue
    text = open(os.path.join(root, name), encoding='utf-8').read()
    for description, body in CASE.findall(text):
        body = re.sub(r'//[^\n]*', '', body)
        octets = bytes(int(value, 0) for value in re.findall(r'\b(0x[0-9a-fA-F]{1,2}|\d+)\b', body))
        print(f'{name}\t{description}\t{octets.hex()}')
