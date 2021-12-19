from dataclasses import dataclass
import re

import numpy as np


INFILE = "input.txt"


@dataclass
class LineSegment:

    x1: int
    y1: int
    x2: int
    y2: int

    @property
    def is_horiz(self):
        return self.y1 == self.y2

    @property
    def is_vert(self):
        return self.x1 == self.x2

    @property
    def is_diag(self):
        return not self.is_horiz and not self.is_vert

    @property
    def xmin(self):
        return min(self.x1, self.x2)

    @property
    def xmax(self):
        return max(self.x1, self.x2)

    @property
    def ymin(self):
        return min(self.y1, self.y2)

    @property
    def ymax(self):
        return max(self.y1, self.y2)


def read_file(filename):
    with open(filename) as f:
        lines = f.readlines()

    segments = []
    for line in lines:
        match = re.fullmatch("(\d+),(\d+) -> (\d+),(\d+)\n", line)
        line_segment = LineSegment(*[int(val) for val in match.groups()])
        segments.append(line_segment)

    return tuple(segments)


def add_segment(grid, segment):
    # convenient shorthand
    l = segment

    if l.is_horiz:
        grid[l.ymin, l.xmin:l.xmax+1] += 1
    elif l.is_vert:
        grid[l.ymin:l.ymax+1, l.xmin] += 1
    elif l.is_diag:
        # sadly slice notation can't do this
        xs = list(range(l.xmin, l.xmax+1))
        ys = list(range(l.ymin, l.ymax+1))
        if l.x1 > l.x2:
            xs = list(reversed(xs))
        if l.y1 > l.y2:
            ys = list(reversed(ys))
        for i, j in zip(xs, ys):
            grid[j, i] += 1
    else:
        raise AssertionError


if __name__ == "__main__":
    segments = read_file(INFILE)

    max_x = max(l.xmax for l in segments)
    max_y = max(l.ymax for l in segments)

    grid = np.zeros((max_x+1, max_y+1), dtype=int)

    for segment in segments:
        add_segment(grid, segment)

    print(f"result: {sum(grid.flatten() > 1)}")
