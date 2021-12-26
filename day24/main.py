import abc
from dataclasses import dataclass
import functools
import re
from typing import Union


INFILE = "input.txt"
MAX_MODEL_NUMBER = int("9"*14)


@dataclass
class Instruction(abc.ABC):

    @abc.abstractmethod
    def process(self, inputs, vars):
        ...

    @staticmethod
    def from_str(line):
        if match := re.fullmatch("inp (\w)", line):
            insn = InputInstruction
        elif match := re.fullmatch("add (\w) (\w|-?\d+)", line):
            insn = AddInstruction
        elif match := re.fullmatch("mul (\w) (\w|-?\d+)", line):
            insn = MulInstruction
        elif match := re.fullmatch("div (\w) (\w|-?\d+)", line):
            insn = DivInstruction
        elif match := re.fullmatch("mod (\w) (\w|-?\d+)", line):
            insn = ModInstruction
        elif match := re.fullmatch("eql (\w) (\w|-?\d+)", line):
            insn = EqlInstruction
        else:
            raise AssertionError

        return insn(*match.groups())


@dataclass
class InputInstruction(Instruction):

    a: str

    def process(self, inputs, vars):
        vars[self.a] = inputs.pop(0)


@dataclass
class OperatorInstruction(Instruction, abc.ABC):

    a: str
    b: Union[str, int]


@dataclass
class AddInstruction(OperatorInstruction):

    def process(self, inputs, vars):
        if self.a == "z":
            print(vars["z"])
        b = vars[self.b] if self.b in vars else int(self.b)
        vars[self.a] += b


@dataclass
class MulInstruction(OperatorInstruction):

    def process(self, inputs, vars):
        b = vars[self.b] if self.b in vars else int(self.b)
        vars[self.a] *= b


@dataclass
class DivInstruction(OperatorInstruction):

    def process(self, inputs, vars):
        b = vars[self.b] if self.b in vars else int(self.b)
        vars[self.a] //= b


@dataclass
class ModInstruction(OperatorInstruction):

    def process(self, inputs, vars):
        b = vars[self.b] if self.b in vars else int(self.b)
        vars[self.a] = vars[self.a] % b


@dataclass
class EqlInstruction(OperatorInstruction):

    def process(self, inputs, vars):
        b = vars[self.b] if self.b in vars else int(self.b)
        vars[self.a] = int(vars[self.a] == b)


def read_file(filename):
    with open(filename) as f:
        return [Instruction.from_str(l.strip()) for l in f.readlines()]


def process_insns(insns, inputs):
    # assert len([i for i in insns if isinstance(i, InputInstruction)]) == len(inputs)

    inputs = inputs.copy()
    vars = {"w": 0, "x": 0, "y": 0, "z": 0}

    for insn in insns:
        try:
            insn.process(inputs, vars)
        except:
            return vars

    return vars


def is_valid_model_number(num, insns):
    inputs = [int(c) for c in str(num)]
    vars = process_insns(insns, inputs)
    return vars["z"]


def test_process_insns():
    insn_str = ["inp w", "add z w", "mod z 2", "div w 2", "add y w",
                "mod y 2", "div w 2", "add x w", "mod x 2", "div w 2", "mod w 2"]
    insns = [Instruction.from_str(i) for i in insn_str]
    vars = process_insns(insns, [11])

    assert list(vars.values()) == [1, 0, 1, 1]

    test_process_insns()


if __name__ == "__main__":
    insns = read_file(INFILE)

    possible_numbers = [0]
    for myval in range(14):
        new_possible_numbers = []
        for num in possible_numbers:
            for i in range(1, 10):
                new_num = int(str(num) + str(i))
                if is_valid_model_number(new_num, insns) < 26**(14-myval):
                    new_possible_numbers.append(new_num)
        possible_numbers = new_possible_numbers
        breakpoint()
        ...

    breakpoint()
    exit()

    is_valid_model_number(39999999999999, insns)
    exit()

    for num in reversed(range(1, MAX_MODEL_NUMBER+1)):
        if "0" in str(num):
            continue

        if is_valid_model_number(num, insns):
            breakpoint()
            ...
