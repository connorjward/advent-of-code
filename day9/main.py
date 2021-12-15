from dataclasses import dataclass
from functools import reduce
from itertools import product
import operator
import sys

import numpy as np


def Minima(*args):
    return Point(*args)

@dataclass(frozen=True)
class Point:

    x: int
    y: int
    val: int


def read_file(filename):
    with open(filename) as f:
        inner_map = [[int(num) for num in line.strip()] for line in f.readlines()]
        inner_map = np.array(inner_map, dtype=int)

    xlen, ylen = inner_map.shape
    height_map = np.full((xlen+2, ylen+2), 1e9, dtype=int)
    height_map[1:-1, 1:-1] = inner_map

    return height_map


def is_low_point(height_map, x, y):
    return all(height_map[x, y] < height_map[i, j]
               for i, j in {(x-1, y), (x+1, y), (x, y-1), (x, y+1)})

def find_low_points(height_map):
    low_points = []
    for (x, y), val in np.ndenumerate(height_map):
        xlen, ylen = height_map.shape
        if x == 0 or x == xlen - 1:
            continue
        if y == 0 or y == ylen - 1:
            continue

        if is_low_point(height_map, x, y):
            low_points.append(Minima(x, y, val))
    return low_points


def find_basin(height_map, low_point):
    checked = {low_point}
    unchecked = get_adjacent_points(height_map, low_point)

    while len(unchecked) > 0:
        pt = unchecked.pop()

        if pt.val < 9:
            checked.add(pt)
            unchecked.update(get_adjacent_points(height_map, pt) - checked)

    return checked


def get_adjacent_points(height_map, pt):
    x, y = pt.x, pt.y
    return {Point(i, j, height_map[i, j]) for i, j in {(x-1, y), (x+1, y), (x, y-1), (x, y+1)}}


def pop_largest_basin(basins):
    largest_basin_idx = 0

    for i, basin in enumerate(basins):
        if len(basin) > len(basins[largest_basin_idx]):
            largest_basin_idx = i

    return basins.pop(largest_basin_idx)


def part1():
    filename = sys.argv[1]

    height_map = read_file(filename)
    low_points = find_low_points(height_map)

    print(f"total risk: {sum(p.val+1 for p in low_points)}")


def part2():
    filename = sys.argv[1]

    height_map = read_file(filename)
    low_points = find_low_points(height_map)

    basins = [find_basin(height_map, p) for p in low_points]

    largest_basins = [pop_largest_basin(basins) for _ in range(3)]

    print(f"final value: {reduce(operator.mul, (len(b) for b in largest_basins))}")


if __name__ == "__main__":
    part2()
