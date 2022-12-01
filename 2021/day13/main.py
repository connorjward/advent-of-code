from dataclasses import dataclass
from enum import IntEnum, auto
import re

import numpy as np


INFILE = "demo.txt"
INFILE = "puzzle.txt"


class FoldDirection(IntEnum):
    LEFT = auto()  # vertical x=?
    UP = auto()    # horizontal y=?


@dataclass
class FoldInstruction:

    loc: int
    direction: FoldDirection



def fold_paper(paper, fold):
    ylen, xlen = paper.shape
    if fold.direction == FoldDirection.LEFT:
        left_overlap = paper[:, 2*fold.loc-xlen+1:fold.loc]
        right = paper[:, fold.loc+1:]

        assert left_overlap.shape == right.shape

        # done in-place
        np.logical_or(left_overlap, right[:, ::-1], out=left_overlap)

        return paper[:, :fold.loc]
    elif fold.direction == FoldDirection.UP:
        top_overlap = paper[2*fold.loc-ylen+1:fold.loc, :]
        bottom = paper[fold.loc+1:, :]

        assert top_overlap.shape == bottom.shape

        # done in-place
        np.logical_or(top_overlap, bottom[::-1, :], out=top_overlap)

        return paper[:fold.loc, :]
    else:
        raise AssertionError
     

def read_file(filename):
    holes = []
    insns = []
    with open(filename) as f:
        for line in f.readlines():
            line = line.strip()

            if len(line) == 0:
                continue

            if not line.startswith("fold along"):
                x, y = line.strip().split(",")
                holes.append((int(x), int(y)))
            else:
                match = re.fullmatch("fold along (x|y)=(\d+)", line)
                dir, val = match.groups()

                if dir == "x":
                    dir = FoldDirection.LEFT
                elif dir == "y":
                    dir = FoldDirection.UP
                else:
                    raise AssertionError

                insns.append(FoldInstruction(int(val), dir))

    xlen = max(x for x, _ in holes) + 1
    ylen = max(y for _, y in holes) + 1
    paper = np.zeros((ylen, xlen), dtype=bool)

    for x, y in holes:
        paper[y, x] = True

    return paper, insns


def print_paper(paper):
    ylen, xlen = paper.shape
    for j in range(ylen):
        for i in range(xlen):
            if paper[j, i]:
                print("#", end="")
            else:
                print(" ", end="")
        print()


if __name__ == "__main__":
    paper, insns = read_file(INFILE)

    for insn in insns:
        paper = fold_paper(paper, insn)

    print_paper(paper)
