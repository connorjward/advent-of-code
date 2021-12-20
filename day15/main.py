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

    def __init__(self, steps, risk, cavern):
        self.steps = steps
        self.risk = risk
        self.score = risk + heuristic(cavern, self)
        self.cavern = cavern

    def __add__(self, step):
        return Path(self.steps + [step], self.risk + step.risk, self.cavern)

    @property
    def last_step(self):
        return self.steps[-1]


def initial_path(cavern):
    # initial position risk not counted
    return Path([Step(0, 0, 0)], 0, cavern)


def search(cavern):
    m, n = cavern.shape
    paths = [initial_path(cavern)]
    scores = [0]

    while True:
        path = paths.pop(0)
        _ = scores.pop(0)

        last_step = path.steps[-1]

        if last_step.x == n-1 and last_step.y == m-1:
            return path

        if last_step.x + 1 < n:
            new_path = path + Step.from_cavern(cavern, last_step.x+1, last_step.y)
            idx = bisection_search(scores, new_path.score)
            paths.insert(idx, new_path)
            scores.insert(idx, new_path.score)
        if last_step.y + 1 < m:
            new_path = path + Step.from_cavern(cavern, last_step.x, last_step.y+1)
            idx = bisection_search(scores, new_path.score)
            paths.insert(idx, new_path)
            scores.insert(idx, new_path.score)


def heuristic(cavern, path):
    m, n = cavern.shape
    return (n - path.last_step.x) + (m - path.last_step.y)


def bisection_search(scores, score):
    return bisect.bisect_right(scores, score)


def find_complete_path(cavern, paths):
    m, n = cavern.shape
    for path in paths:
        if path.final_step.x == m-1 and path.final_step.y == n-1:
            return path
    return None


def select_next_path(cavern, possible_paths):
    return min(possible_paths, key=lambda p: p.score)


def read_file(filename):
    with open(filename) as f:
        cavern = [[int(c) for c in l.strip()] for l in f.readlines()]
        return np.array(cavern, dtype=int)


if __name__ == "__main__":
    cavern = read_file(INFILE)

    best_path = search(cavern)
    print(f"final value: {best_path.risk}")
