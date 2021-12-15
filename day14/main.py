from collections import Counter
from dataclasses import dataclass
from functools import lru_cache
import re
import time


# INFILE = "demo.txt"
INFILE = "puzzle.txt"
NSTEPS = 40


class Polymeriser:

    def __init__(self, template, rules):
        self._template = template
        self._rules = rules
        self._counter = Counter()

    def polymerise(self, nsteps):
        for e1, e2 in zip(self._template, self._template[1:]):
            self._counter += self._polymerise(e1, e2, nsteps) 
        return self._counter

    @classmethod
    def from_file(cls, filename):
        with open(filename) as f:
            template = [c for c in f.readline().strip()]

            # discard empty line
            f.readline()

            rules = {}
            for line in f.readlines():
                match = re.fullmatch("(\w)(\w) -> (\w)", line.strip())
                left, right, inner = match.groups()
                rules[left, right] = (left, inner), (inner, right)

        return cls(template, rules)
        
    # counter is really slow...
    @lru_cache
    def _polymerise(self, elem1, elem2, depth):
        (e1, e2), (_, e3) = self._rules[elem1, elem2]

        if depth == 1:
            return Counter([e1, e2, e3])
        else:
            counter = self._polymerise(e1, e2, depth-1) + self._polymerise(e2, e3, depth-1)
            counter[e2] -= 1  # remove duplicate
            return counter
            
            
if __name__ == "__main__":
    polymeriser = Polymeriser.from_file(INFILE)
    counter = polymeriser.polymerise(NSTEPS)

    count = counter.most_common()
    most_common = count[0]
    least_common = count[-1]

    print(f"final value: {most_common[1]-least_common[1]}")
