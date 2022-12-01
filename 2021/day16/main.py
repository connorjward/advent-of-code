from abc import abstractmethod
from dataclasses import dataclass
from functools import reduce
import operator


# INFILE = "demo.txt"
INFILE = "input.txt"


@dataclass
class PacketIterator:

    contents: str
    index: int = 0

    def next(self, n=1):
        substr = "".join([self.contents[self.index:self.index+n]])
        self.index += n
        return substr

    @classmethod
    def from_file(cls, filename):
        with open(filename) as f:
            return cls(hex2bin(f.readline()))


@dataclass
class Packet:

    version: int
    type_id: int


@dataclass
class OperatorPacket(Packet):

    subpackets: list

    @property
    @abstractmethod
    def value(self):
        ...

    @staticmethod
    def create(version, type_id, subpackets):
        if type_id == 0:
            cls = SumPacket
        elif type_id == 1:
            cls = ProductPacket
        elif type_id == 2:
            cls = MinPacket
        elif type_id == 3:
            cls = MaxPacket
        elif type_id == 5:
            cls = GreaterThanPacket
        elif type_id == 6:
            cls = LessThanPacket
        elif type_id == 7:
            cls = EqualToPacket
        else:
            raise AssertionError

        return cls(version, type_id, subpackets)


class SumPacket(OperatorPacket):

    @property
    def value(self):
        return sum(p.value for p in self.subpackets)


class ProductPacket(OperatorPacket):

    @property
    def value(self):
        return reduce(operator.mul, (p.value for p in self.subpackets))


class MinPacket(OperatorPacket):

    @property
    def value(self):
        return min(p.value for p in self.subpackets)


class MaxPacket(OperatorPacket):

    @property
    def value(self):
        return max(p.value for p in self.subpackets)


class GreaterThanPacket(OperatorPacket):

    @property
    def value(self):
        p1, p2 = self.subpackets
        return int(p1.value > p2.value)


class LessThanPacket(OperatorPacket):

    @property
    def value(self):
        p1, p2 = self.subpackets
        return int(p1.value < p2.value)


class EqualToPacket(OperatorPacket):

    @property
    def value(self):
        p1, p2 = self.subpackets
        return int(p1.value == p2.value)


@dataclass
class LiteralPacket(Packet):

    value: int


def parse_subpacket(packet):
    version = bin2dec(packet.next(3))
    type_id = bin2dec(packet.next(3))

    if type_id == 4:
        return LiteralPacket(version, type_id, parse_literal(packet))
    else:
        return OperatorPacket.create(version, type_id, parse_operator(packet))
        

def parse_literal(packet):
    literal = []
    while (prefix_bit := packet.next()) == "1":
        literal.append(packet.next(4))
    literal.append(packet.next(4))
    return bin2dec("".join(literal))


def parse_operator(packet):
    subpackets = []

    length_type_id = packet.next()
    if length_type_id == "0":
        subpacket_len = bin2dec(packet.next(15))

        start_idx = packet.index
        while packet.index < start_idx + subpacket_len:
            subpackets.append(parse_subpacket(packet))
    elif length_type_id == "1":
        nsubpackets = bin2dec(packet.next(11))

        for _ in range(nsubpackets):
            subpackets.append(parse_subpacket(packet))
    else:
        raise AssertionError
    return subpackets


def hex2bin(val):
    return bin(int(val, 16))[2:]


def bin2dec(val):
    return int(val, 2)


def version_sum(packet):
    if isinstance(packet, OperatorPacket):
        return packet.version + sum(version_sum(p) for p in packet.subpackets)
    else:
        return packet.version


if __name__ == "__main__":
    packetiter = PacketIterator.from_file(INFILE)
    packet = parse_subpacket(packetiter)

    print(f"final value: {packet.value}")
