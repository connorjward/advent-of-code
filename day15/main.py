import bisect
from functools import cached_property

import numpy as np
# need to do depth first search. get leaves with shortest current path.

# assume only down and right
# INFILE = "demo.txt"
INFILE = "puzzle.txt"


class Step:

    def __init__(self, x, y, risk):
        self.x = x
        self.y = y
        self.risk = risk

    @classmethod
    def from_cavern(cls, cavern, x, y):
        return cls(x, y, cavern[y, x])


class Path:

    def __init__(self, steps, risk):
        self.steps = steps
        self.risk = risk

    def __add__(self, step):
        return Path(self.steps + [step], self.risk + step.risk)

    @property
    def final_step(self):
        return self.steps[-1]


def initial_path(cavern):
    # initial position risk not counted
    return Path([Step(0, 0, 0)], 0)


def search(cavern):
    m, n = cavern.shape
    paths = [initial_path(cavern)]
    risks = [0]

    while True:
        path = paths.pop(0)
        _ = risks.pop(0)  # discard value

        last_step = path.steps[-1]

        # if we are above or left of the final value then we can stop here
        if (last_step.x == n-1 and last_step.y == m-2
                or last_step.x == n-2 and last_step.y == m-1):
            return path + Step.from_cavern(cavern, n-1, m-1)

        if last_step.x + 1 < n:
            new_path = path + Step.from_cavern(cavern, last_step.x+1, last_step.y)
            idx = bisection_search(risks, new_path.risk)
            paths.insert(idx, new_path)
            risks.insert(idx, new_path.risk)
        if last_step.y + 1 < m:
            new_path = path + Step.from_cavern(cavern, last_step.x, last_step.y+1)
            idx = bisection_search(risks, new_path.risk)
            paths.insert(idx, new_path)
            risks.insert(idx, new_path.risk)


def bisection_search(risks, risk):
    return bisect.bisect_right(risks, risk)


def find_complete_path(cavern, paths):
    m, n = cavern.shape
    for path in paths:
        if path.final_step.x == m-1 and path.final_step.y == n-1:
            return path
    return None


def select_next_path(possible_paths):
    return min(possible_paths, key=lambda p: p.risk)


def read_file(filename):
    with open(filename) as f:
        cavern = [[int(c) for c in l.strip()] for l in f.readlines()]
        return np.array(cavern, dtype=int)


if __name__ == "__main__":
    cavern = read_file(INFILE)

    best_path = search(cavern)
    print(f"final value: {best_path.risk}")
