#!/usr/bin/env python3
"""The GTPv2-C messages of a capture of lab runs, as tests/captures/messages.tsv has them:
each UDP datagram to or from port 2123 of a capture in the format of libpcap, one line
each, by the run, its number in the run, its source, its destination, the interface (S11
between the MME and the SGW-C, S5 between the SGW-C and the PGW-C, by the addresses of
the lab) and its octets in hexadecimal.

    python3 tests/captures/extract.py RUN CAPTURE MME,SGW-C,PGW-C >> tests/captures/messages.tsv
"""
import ipaddress
import struct
import sys


def frames(data):
    magic = data[:4]
    order = {b'\xd4\xc3\xb2\xa1': '<', b'\xa1\xb2\xc3\xd4': '>', b'\x4d\x3c\xb2\xa1': '<', b'\xa1\xb2\x3c\x4d': '>'}.get(magic)
    if order is None:
        sys.exit('not a capture of libpcap')
    linktype = struct.unpack(order + 'I', data[20:24])[0]
    at = 24
    while at + 16 <= len(data):
        _, _, length, _ = struct.unpack(order + 'IIII', data[at:at + 16])
        yield linktype, data[at + 16:at + 16 + length]
        at += 16 + length


def datagram(linktype, frame):
    """The source, the destination, the ports and the payload of an IPv4 UDP frame."""
    if linktype == 1:  # Ethernet
        if frame[12:14] != b'\x08\x00':
            return None
        packet = frame[14:]
    elif linktype == 113:  # Linux cooked
        packet = frame[16:]
    elif linktype == 101:  # raw IP
        packet = frame
    else:
        sys.exit(f'link type {linktype}')
    if packet[0] >> 4 != 4 or packet[9] != 17:
        return None
    header = (packet[0] & 0x0f) * 4
    source, destination = ipaddress.ip_address(packet[12:16]), ipaddress.ip_address(packet[16:20])
    sport, dport, length = struct.unpack('>HHH', packet[header:header + 6])
    return str(source), str(destination), sport, dport, packet[header + 8:header + length]


def main():
    run, capture, nodes = sys.argv[1], sys.argv[2], sys.argv[3].split(',')
    mme, sgw, pgw = nodes
    number = 0
    for linktype, frame in frames(open(capture, 'rb').read()):
        found = datagram(linktype, frame)
        if found is None or 2123 not in found[2:4]:
            continue
        source, destination, _, _, payload = found
        pair = {source, destination}
        interface = 'S11' if pair == {mme, sgw} else 'S5' if pair == {sgw, pgw} else '?'
        number += 1
        print('\t'.join([run, str(number), source, destination, interface, payload.hex()]))


main()
