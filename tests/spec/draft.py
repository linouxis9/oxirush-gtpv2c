#!/usr/bin/env python3
"""Draft the registry of the crate from the fixtures of TS 29.274 (tests/spec/*.tsv, with
errata.tsv): the IE types (src/registry_tables.rs, one line each), the tables of clause 7
(one row each), the message types (src/message_types.rs), and a layout of the octets of
each IE type that src/values.rs does not lay out (src/values_octets.rs). The draft is
reviewed and formatted with `cargo fmt`, and the test `spec` compares the crate with the
fixtures.

    python3 tests/spec/draft.py . 1 2 3 ...   # the IE types that src/values.rs lays out
"""
import re
import sys
from collections import OrderedDict

crate = sys.argv[1]
laid_out = set(int(t) for t in sys.argv[2:])
spec = f'{crate}/tests/spec'


def tsv(name):
    return [line.rstrip('\n').split('\t') for line in open(f'{spec}/{name}', encoding='utf-8') if not line.startswith('#')]


def pascal(text):
    words = re.findall(r'[A-Za-z0-9]+', text)
    out = ''.join(w[:1].upper() + w[1:].lower() for w in words)
    return re.sub(r'^(\d)', r'N\1', out)


KEYWORDS = {'type', 'ref', 'match', 'self', 'mod', 'use', 'in', 'as', 'fn', 'move', 'extra', 'order'}


def snake(text):
    words = re.findall(r'[A-Za-z0-9]+', text.replace('(e)', 'e'))
    out = '_'.join(w.lower() for w in words)
    out = re.sub(r'^(\d)', r'n\1', out)
    return out + '_' if out in KEYWORDS else out


def norm(text):
    return re.sub(r'[^a-z0-9]', '', text.lower())


errata = {(kind, table): read for kind, table, _, read, _ in tsv('errata.tsv') if kind in ('number', 'grouped')}
names = {written: read for kind, _, written, read, _ in tsv('errata.tsv') if kind == 'ie-name'}
aliases = {written: int(read) for kind, _, written, read, _ in tsv('errata.tsv') if kind == 'ie-type'}

# The IE types of Table 8.1-1
types = OrderedDict()
for number, name, reference, fixed in tsv('ie-types.tsv'):
    if not number.isdigit() or name.startswith(('Reserved', 'Spare')):
        continue
    name = names.get(name, name)
    found = re.match(r'(Fixed Length|Variable Length|Variable|Extendable)\s*/\s*(\S+)', reference)
    other = re.search(r'TS (\d+\.\d+)', reference)
    if found:
        form, clause = {'Fixed Length': 'fixed', 'Variable Length': 'variable', 'Variable': 'variable', 'Extendable': 'extendable'}[found.group(1)], found.group(2)
    elif other:
        form, clause = 'variable', 'TS ' + other.group(1)
    else:
        continue
    fixed = fixed.strip() if re.fullmatch(r'\d+', fixed.strip()) else ''
    short = re.search(r'\(([^)]*)\)\s*$', name)
    short = short.group(1) if short else None
    # An abbreviation in capitals names the type (IMSI, F-TEID); another the name without it
    acronym = short and re.fullmatch(r'[A-Z0-9][A-Z0-9a-z/-]*', short) and sum(c.isupper() for c in short) >= 2
    rust = pascal(short) if acronym else pascal(re.sub(r'\s*\([^)]*\)\s*$', '', name))
    if name.startswith('MM Context'):
        # Six types of one name: MmContextEpsSecurityContextAndQuadruplets
        rust = pascal(name)
    types[int(number)] = dict(name=name, form=form, clause=clause, fixed=fixed, rust=rust, short=short)

# What the tables name an IE type by
by_name = {}
for number, info in types.items():
    by_name.setdefault(norm(info['name']), number)
    if info['short']:
        by_name.setdefault(norm(info['short']), number)
    by_name.setdefault(norm(re.sub(r'\s*\([^)]*\)\s*$', '', info['name'])), number)
for alias, number in aliases.items():
    by_name.setdefault(norm(alias), number)


def type_of(cell):
    cell = re.sub(r'\s*\((?:See )?NOTE[^)]*\)', '', cell).strip()
    for key in (norm(cell), norm(re.sub(r'\s*\([^)]*\)\s*$', '', cell))):
        if key in by_name:
            return by_name[key]
    sys.exit(f'no IE type of the name {cell!r}')


PRESENCE_RANK = {'M': 0, 'C': 1, 'CO': 2, 'O': 3}

# The tables of clause 7, a row of several lines (the presences of other interfaces) once,
# of its weakest presence where the lines differ, and mandatory where each line is
tables = OrderedDict()
for line in tsv('tables.tsv'):
    if line[0] == 'table':
        _, number, caption, prelude = line[:4]
        prelude = errata.get(('grouped', number), prelude)
        number = errata.get(('number', number), number)
        tables[number] = dict(caption=caption, prelude=prelude, rows=[], number=number)
        current = tables[number]
    elif line[0] in ('row', 'also'):
        _, _, name, presence, ie_type, instance, several = line
        # A note of a name is no part of it: "Additional Protocol Configuration Options (APCO) (NOTE 29)"
        name = re.sub(r'\s*\((?:See )?NOTE[^)]*\)', '', name).strip()
        rows = current['rows']
        if instance == 'VS':
            continue
        t = type_of(ie_type)
        last = rows[-1] if rows else None
        # A line of a merged row may leave its instance empty: the row's
        if not instance and last and last['name'] == name:
            instance = last['instance']
        if not instance.isdigit():
            sys.exit(f'no instance of {name} in Table {current["number"]}')
        if last and (last['name'], last['type'], last['instance']) == (name, t, instance):
            last['presences'].append(presence)
            last['several'] = last['several'] or several == 'several'
            continue
        rows.append(dict(name=name, presences=[presence], type=t, instance=instance, several=several == 'several'))
for table in tables.values():
    for row in table['rows']:
        ps = row['presences']
        row['presence'] = 'M' if all(p == 'M' for p in ps) else max((p for p in ps if p != 'M'), key=PRESENCE_RANK.get, default='C')
        if 'M' in ps and row['presence'] != 'M':
            row['presence'] = 'C'

order = list(tables)
grouped_types = {int(t['prelude']) for t in tables.values() if t['prelude'].isdigit()}

message_numbers = {}
for number, name, *_ in tsv('message-types.tsv'):
    message_numbers[norm(re.sub(r'\s*\(.*$', '', name))] = int(number)
message_numbers.setdefault(norm('Configuration Transfer Tunnel Message'), message_numbers[norm('Configuration Transfer Tunnel')])


def message_of(table):
    caption = re.sub(r'^Information Elements? in (an? )?', '', table['caption'])
    return caption, message_numbers.get(norm(caption))


def words(text):
    return set(re.findall(r'[a-z0-9]+', text.lower().replace('contexts', 'context'))) - {
        'within', 'the', 'a', 'an', 'in', 'of', 'ie', 'information', 'elements', 'element', 'with'}


def child_table(parent, row):
    """The table of the content of a grouped row of `parent`: a table of its IE type after
    the parent's and before the next message's, whose caption is nearest the row's name."""
    candidates = []
    for number in order[order.index(parent) + 1:]:
        table = tables[number]
        if not table['prelude'].isdigit():
            if message_of(table)[1] is not None:
                break
            continue
        if int(table['prelude']) == row['type']:
            candidates.append(number)
    if not candidates:
        # One of its clause, or another's of the same type (Overload Control Information)
        clause = parent.split('-')[0] + '-'
        same = [n for n in order if tables[n]['prelude'] == str(row['type'])]
        candidates = [n for n in same if n.startswith(clause)] or same
    if not candidates:
        return None
    return max(candidates, key=lambda n: (len(words(tables[n]['caption']) & words(row['name'])), -order.index(n)))


rust_names, used = {}, {t['rust'] for t in types.values()}
for number in order:
    table = tables[number]
    caption = re.sub(r'^Information Elements? in (an? )?', '', table['caption'])
    caption = re.sub(r'\s+with(in)?\s+', ' in ', caption)
    base = pascal(caption)
    name = base
    if name in used:
        name = base + 'T' + re.sub(r'\W', '_', number)
    used.add(name)
    rust_names[number] = name

placed = {}
messages = []
blocks = []
for number in order:
    table = tables[number]
    if table['prelude'].isdigit():
        code = int(table['prelude'])
        head = f'    group {rust_names[number]} = {code}, "{types[code]["name"]}", "{number}" {{'
    else:
        message, code = message_of(table)
        if code is None:
            sys.exit(f'no message type of Table {number}: {table["caption"]}')
        messages.append((code, rust_names[number]))
        head = f'    message {rust_names[number]} = {code}, "{message}", "{number}" {{'
    lines = ['gtpv2c_table! {', f'    /// {table["caption"]} (Table {number}).', head]
    fields = set()
    for row in table['rows']:
        field = snake(re.sub(r'\s*\([^)]*\)', '', row['name']))
        if field in fields:
            field = f'{field}_{row["instance"]}'
        while field in fields:
            field += '_'
        fields.add(field)
        if row['type'] in grouped_types:
            child = child_table(number, row)
            if child is None:
                sys.exit(f'no table of the content of {row["name"]} in Table {number}')
            placed.setdefault(child, number)
            ty = rust_names[child]
        else:
            ty = types[row['type']]['rust']
        repeat = '*' if row['several'] else ''
        name = row['name'].replace('"', '\\"')
        lines.append(f'        {field}: "{name}" {row["presence"]}{repeat} {ty} {row["instance"]} [],')
    lines += ['    }', '}', '']
    blocks.append('\n'.join(lines))

unplaced = [n for n in order if tables[n]['prelude'].isdigit() and n not in placed]
for number in unplaced:
    print('a table of no row:', number, tables[number]['caption'], file=sys.stderr)

contents = OrderedDict()
for number in order:
    if tables[number]['prelude'].isdigit():
        contents.setdefault(int(tables[number]['prelude']), []).append(rust_names[number])

out = ['gtpv2c_ies! {']
for number, info in types.items():
    if number in grouped_types:
        out.append(f'    {number} "{info["name"]}" grouped "{info["clause"]}" => {", ".join(contents[number])};')
    else:
        fixed = f'({info["fixed"]})' if info['fixed'] else ''
        out.append(f'    {number} "{info["name"]}" {info["form"]}{fixed} "{info["clause"]}" => {info["rust"]};')
out += ['}', '']
out += blocks
open(f'{crate}/src/registry_tables.rs', 'w').write('\n'.join(out) + '\n')
initial = sorted(int(n) for n, name, first, *_ in tsv('message-types.tsv') if first.strip() == 'X' and n.isdigit())
listed = ['/// Whether a message type is an initial message of Table 6.1-1, which its receiver answers:',
          '/// a request, a command or a notification. A type that the table does not have is none.',
          'pub fn is_initial(message_type: u8) -> bool {',
          '    matches!(message_type, ' + ' | '.join(map(str, initial)) + ')', '}', '']
with_cause = []
for number in order:
    table = tables[number]
    if table['prelude'].isdigit() or message_of(table)[1] is None:
        continue
    for row in table['rows']:
        if row['type'] == 2 and row['instance'] == '0' and not row['several']:
            with_cause.append((rust_names[number], snake(re.sub(r'\s*\([^)]*\)', '', row['name']))))
listed += ['impl Body {', '    /// The Cause of the message at its level, of instance 0.',
           '    pub fn cause(&self) -> Option<CauseValue> {', '        let cause = match self {']
listed += [f'            Self::{ty}(body) => &body.{field},' for ty, field in with_cause]
listed += ['            _ => return None,', '        };', '        cause.as_ref().map(|cause| cause.value)', '    }', '}', '']
listed += ['gtpv2c_messages! {'] + [f'    {code} {name} [],' for code, name in sorted(messages)] + ['}']
open(f'{crate}/src/message_types.rs', 'w').write('\n'.join(listed) + '\n')

vals = ['gtpv2c_layout! {']
for number, info in types.items():
    if number in grouped_types or number in laid_out:
        continue
    vals.append(f'    /// {info["name"]} (§{info["clause"]}): its octets, which the crate does not split.')
    # The octets of an MM Context hold keys, which `Debug` does not print
    kind = 'SecretRest' if 103 <= number <= 108 else 'Rest'
    vals.append(f'    pub struct {info["rust"]} "{info["clause"]}" {{ fields {{ octets "{info["name"]}": {kind} }} }}')
vals.append('}')
values = [types[n]['rust'] for n in types if n not in grouped_types]
vals.append('')
vals.append('listed_value!(')
vals += [f'    {v},' for v in values]
vals.append(');')
open(f'{crate}/src/values_octets.rs', 'w').write('\n'.join(vals) + '\n')
print(f'{len(types)} IE types, {len(tables)} tables, {sum(len(t["rows"]) for t in tables.values())} rows, {len(messages)} messages', file=sys.stderr)
