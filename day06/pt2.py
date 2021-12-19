from collections import defaultdict
from dataclasses import dataclass
import sys

NDAYS = 256
NEW_BREED_TIME = 8


@dataclass
class FishGeneration:

    count: int
    timer: int = NEW_BREED_TIME


def read_file(filename):
    with open(filename) as f:
        vals = [int(val) for val in f.readline().split(",")]

    ctr = defaultdict(int)
    for val in vals:
        ctr[val] += 1

    gens = []
    for timer, count in ctr.items():
        gens.append(FishGeneration(count, timer))
    return gens


def breed(fish_gens):
    new_fish_count = 0
    for fish_gen in fish_gens:
        if fish_gen.timer == 0:
            fish_gen.timer = 6
            new_fish_count += fish_gen.count
        else:
            fish_gen.timer -= 1

    fish_gens.append(FishGeneration(new_fish_count))


if __name__ == "__main__":
    infile = sys.argv[1]
    fish_gens = read_file(infile)

    for _ in range(NDAYS):
        breed(fish_gens)

    print(f"final fish count: {sum(g.count for g in fish_gens)}")
