from collections import defaultdict


START = "start"
END = "end"

# INFILE = "demo.txt"
INFILE = "puzzle.txt"


def load_caves(filename):
    caves = defaultdict(list)
    with open(filename) as f:
        for line in f.readlines():
            cave1, cave2 = line.strip().split("-")
            caves[cave1].append(cave2)
            caves[cave2].append(cave1)
    return caves


def possible_moves(caves, start_cave, seen_small_caves):
    return tuple(c for c in caves[start_cave] if c not in seen_small_caves)


def search(caves, start=START, seen_small_caves=None):
    if seen_small_caves is None:
        seen_small_caves = [start]

    paths = [[start]]
    for dest in possible_moves(caves, start, seen_small_caves):
        if is_small_cave(dest):
            seen_small_caves.append(dest)

        paths_ = search(caves, dest, seen_small_caves)

        for p in paths_:
            paths.append([start] + p)

        if is_small_cave(dest):
            seen_small_caves.pop()

    # only return paths that actually end in the right place
    return tuple(p for p in paths if p[-1] == END)


def is_small_cave(cave):
    return cave.islower()


if __name__ == "__main__":
    caves = load_caves(INFILE)
    paths = search(caves)

    print(len(paths))
