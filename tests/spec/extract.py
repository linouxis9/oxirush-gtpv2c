#!/usr/bin/env python3
"""The fixtures of the test `spec`, from the document that the crate follows: TS 29.274
V19.6.0 (2026-03), as the 3GPP archive has it.

    python3 tests/spec/extract.py 29274-j60.zip     # the archive, fetched already
    python3 tests/spec/extract.py --fetch           # fetched from the 3GPP archive

The archive is checked against its SHA-256, and the fixtures are written beside this
script:

    message-types.tsv   Table 6.1-1: the message types, whether each is initial or
                        triggered, and the S3, S10, S16 and N26 columns of those that have
                        them
    ie-types.tsv        Table 8.1-1: the IE types, their references and their lengths
    comments.tsv        the condition or comment of each line of a row of clause 7, by its
                        table and the name of its row, which the conditions that the crate
                        checks (conditions.tsv) are compared with
    tables.tsv          the tables of clause 7, one row each: a table by its number, its
                        caption and, for a grouped IE, the IE type of its first lines; a
                        row by its name, its presence, its IE type and its instance, and
                        whether its comment says that several IEs of it may be there. A
                        line of a row without a name is a second presence of the row above
                        on other interfaces, `also`

The document is read by tools/docx_tables.py of the oxirush-utils that the crate builds
with. The cells are written as the document has them, with their spaces made single:
what the crate does differently is in errata.tsv.
"""
import json
import os
import re
import subprocess
import sys

URL = 'https://www.3gpp.org/ftp/Specs/archive/29_series/29.274/29274-j60.zip'
SHA256 = '6e9d8e04f80c6454b77562966a099a0d35b4eccf1c1db1b83a151d5555754c88'
HERE = os.path.dirname(os.path.abspath(__file__))


def utils_tools():
    """The tools of the oxirush-utils that the crate builds with, which read the document."""
    listed = ['cargo', 'metadata', '--format-version', '1']
    crate = os.path.dirname(os.path.dirname(HERE))
    metadata = json.loads(subprocess.run(listed, cwd=crate, check=True, capture_output=True, text=True).stdout)
    package = next(p for p in metadata['packages'] if p['name'] == 'oxirush-utils')
    return os.path.join(os.path.dirname(package['manifest_path']), 'tools')


sys.path.insert(0, utils_tools())
import docx_tables  # noqa: E402
from docx_tables import blocks, clean  # noqa: E402

PRESENCES = {'M', 'C', 'CO', 'O'}
# A caption as the document writes it, its number too: "Table 7.4.z-1: ...", "Table 7.9.2: ..."
CAPTION = re.compile(r'Table (\d+(?:\.[\dA-Za-z]+)*(?:-\d+)?)\s*:\s*(.*)')
# "Several IEs with the same type and instance value may be included", and its kin
SEVERAL = re.compile(r'(?i)several IEs?\b|more than one IE|represent a list|IEs? with the same type and instance')


def archive():
    if sys.argv[1:] == ['--fetch']:
        data = docx_tables.fetch(URL)
    elif len(sys.argv) == 2:
        data = open(sys.argv[1], 'rb').read()
    else:
        sys.exit(__doc__)
    return docx_tables.document(data, SHA256)


def write(name, head, rows):
    with open(os.path.join(HERE, name), 'w', encoding='utf-8') as out:
        out.write('# ' + '\t'.join(head) + '\n')
        for row in rows:
            out.write('\t'.join(cell.replace('\t', ' ').replace('\n', ' ') for cell in row) + '\n')
    print(f'{name}: {len(rows)} lines')


def message_types(rows):
    """The rows of Table 6.1-1: a number, a name, initial and triggered, and the columns of
    the interfaces of S3, S10, S16 and N26 where the table has them."""
    out = []
    for row in rows:
        cells = [clean(cell) for cell, _ in row]
        if not cells or not re.fullmatch(r'\d+', cells[0]):
            continue
        if len(cells) >= 9:
            # 128 | name | S3 | S10 | S16 | N26 | (empty) | initial | triggered
            number, name, s3, s10, s16, n26 = cells[:6]
            initial, triggered = cells[-2], cells[-1]
            out.append([number, name, initial, triggered, s3, s10, s16, n26])
        else:
            number, name = cells[0], cells[1]
            initial = cells[3] if len(cells) > 3 else ''
            triggered = cells[4] if len(cells) > 4 else ''
            out.append([number, name, initial, triggered, '', '', '', ''])
    return out


def ie_types(rows):
    out = []
    for row in rows:
        cells = [clean(cell) for cell, _ in row]
        if len(cells) >= 4 and re.fullmatch(r'\d+', cells[0]):
            out.append(cells[:4])
    return out


def table_rows(number, caption, prelude, rows, comments):
    """A table of clause 7 and its rows, and the comment of each line of a row."""
    out = [['table', number, caption, prelude]]
    last = None
    for row in rows:
        cells = [clean(cell) for cell, _ in row]
        if len(cells) < 5 or cells[1] not in PRESENCES:
            continue
        name, presence, comment, ie_type, instance = cells[:5]
        several = 'several' if SEVERAL.search(comment) else 'once'
        if name or last is not None:
            comments.append([number, name or last, presence, comment])
        if name:
            out.append(['row', number, name, presence, ie_type, instance, several])
            last = name
        elif last is not None:
            out.append(['also', number, last, presence, ie_type, instance, several])
    return out


def main():
    body = archive()
    caption, heading = '', ''
    messages, types, tables, comments = [], [], [], []
    for kind, block in blocks(body):
        if kind == 'p':
            style, value = block
            if style.startswith('Heading'):
                heading = value
            # A caption of the style TH, or of no style: "Table 7.3.1-4: Remote UE ..."
            if style == 'TH' or CAPTION.match(value):
                caption = value
            continue
        rows, _ = block
        found = CAPTION.match(caption)
        caption_of_this, caption = caption, ''
        if found and found.group(1) == '6.1-1':
            messages = message_types(rows)
            continue
        if found and found.group(1) == '8.1-1':
            types = ie_types(rows)
            continue
        # A grouped IE's table starts with lines of its IE type, its length and its instance
        prelude = ''
        for row in rows[:3]:
            for cell, _ in row:
                typed = re.search(r'IE Type\s*=\s*(\d+|x)', clean(cell))
                if typed:
                    prelude = typed.group(1)
        if not any(len(row) >= 5 and clean(row[1][0]) in PRESENCES for row in rows if len(row) > 1):
            continue
        if not heading.startswith('7'):
            continue
        number = found.group(1) if found else f'(no caption) after {heading.split()[0]}'
        title = found.group(2) if found else ''
        tables.extend(table_rows(number, title, prelude, rows, comments))
    write('message-types.tsv', ['type', 'name', 'initial', 'triggered', 'S3', 'S10', 'S16', 'N26'], messages)
    write('ie-types.tsv', ['type', 'name', 'reference', 'number of fixed octets'], types)
    write('comments.tsv', ['table', 'row', 'presence', 'condition or comment'], comments)
    write('tables.tsv', ['table|row|also', 'table', 'caption, or the name of the row', 'IE type of the grouped IE, or presence', 'IE type', 'instance', 'once or several'], tables)


main()
