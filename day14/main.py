from collections import Counter, defaultdict
from dataclasses import dataclass
from functools import lru_cache
import re
import time

import numpy as np


# INFILE = "demo.txt"
INFILE = "puzzle.txt"
# NSTEPS = 10
NSTEPS = 40


def read_file(filename):
    with open(filename) as f:
        template = [c for c in f.readline().strip()]

        # discard empty line
        f.readline()

        rules = {}
        for line in f.readlines():
            match = re.fullmatch("(\w)(\w) -> (\w)", line.strip())
            left, right, inner = match.groups()
            rules[left, right] = (left, inner), (inner, right)

    return template, rules


def template2vector(template, rules):
    m = len(rules)
    vec = np.zeros((m), dtype=int)

    for pair in zip(template, template[1:]):
        vec[list(rules.keys()).index(pair)] += 1

    return vec


def vector2dict(vec, rules):
    return {key: i for i, (key, _) in enumerate(rules.items())}


def count_occurrences(vec, rules, end_elem):
    tuple_dict = vector2dict(vec, rules)

    counter = defaultdict(int)
    for (e1, _), idx in tuple_dict.items():
        # only do e1 since e2 is duplicated
        counter[e1] += vec[idx]

    # add end entry
    counter[end_elem] += 1

    return counter


def rules2matrix(rules):
    m = len(rules)
    mat = np.zeros((m, m), dtype=int)

    for i, (key, val) in enumerate(rules.items()):
        pair1, pair2 = val
        mat[list(rules.keys()).index(pair1), i] = 1
        mat[list(rules.keys()).index(pair2), i] = 1

    return mat
            
            
if __name__ == "__main__":
    template, rules = read_file(INFILE)

    mat = rules2matrix(rules)
    vec = template2vector(template, rules)

    for _ in range(NSTEPS):
        vec = mat @ vec

    counter = count_occurrences(vec, rules, template[-1])

    print(f"result: {max(counter.values())-min(counter.values())}")
