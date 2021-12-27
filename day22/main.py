from dataclasses import dataclass, field
import itertools
import re

import numpy as np


INFILE = "input.txt"


@dataclass(frozen=True)
class Range:

    min: int
    max: int

    @property
    def is_zero_sized(self):
        return self.min > self.max

    @property
    def size(self):
        return self.max - self.min

    @classmethod
    def compute_overlap(cls, range1, range2):
        min_ = max(range1.min, range2.min)
        max_ = min(range1.max, range2.max)
        return cls(min_, max_)


@dataclass(frozen=True)
class BoundingBox:

    x: Range
    y: Range
    z: Range

    @property
    def is_zero_sized(self):
        return any(r.is_zero_sized for r in [self.x, self.y, self.z])

    @property
    def size(self):
        assert not self.is_zero_sized

        return (self.x.max+1-self.x.min)*(self.y.max+1-self.y.min)*(self.z.max+1-self.z.min)

    @classmethod
    def compute_overlap(cls, extent1, extent2):
        xrange = Range.compute_overlap(extent1.x, extent2.x)
        yrange = Range.compute_overlap(extent1.y, extent2.y)
        zrange = Range.compute_overlap(extent1.z, extent2.z)
        return cls(xrange, yrange, zrange)


@dataclass
class CubeList:

    bounds: BoundingBox
    status: bool

    def subtract(self, other):
        # split into 26 parts around the cutout
        overlap = BoundingBox.compute_overlap(self.bounds, other.bounds)

        if BoundingBox.compute_overlap(overlap, self.bounds).is_zero_sized:
            raise AssertionError

        split_cubelists = []

        xranges = [Range(self.bounds.x.min, overlap.x.min-1),
                   Range(overlap.x.min, overlap.x.max),
                   Range(overlap.x.max+1, self.bounds.x.max)]
        yranges = [Range(self.bounds.y.min, overlap.y.min-1),
                   Range(overlap.y.min, overlap.y.max),
                   Range(overlap.y.max+1, self.bounds.y.max)]
        zranges = [Range(self.bounds.z.min, overlap.z.min-1),
                   Range(overlap.z.min, overlap.z.max),
                   Range(overlap.z.max+1, self.bounds.z.max)]
        
        for i, xrange in enumerate(xranges):
            for j, yrange in enumerate(yranges):
                for k, zrange in enumerate(zranges):
                    # this is the cutout section
                    if i == 1 and j == 1 and k == 1:
                        continue
                    box = BoundingBox(xrange, yrange, zrange)

                    if not box.is_zero_sized:
                        split_cubelists.append(type(self)(box, self.status))

        for b1, b2 in itertools.permutations(split_cubelists, 2):
            assert BoundingBox.compute_overlap(b1.bounds, b2.bounds).is_zero_sized

        return split_cubelists


CENTER = BoundingBox(Range(-50, 50), Range(-50, 50), Range(-50, 50))


def parse_line(line):
    pattern = "(\S+) x=(-?\d+)..(-?\d+),y=(-?\d+)..(-?\d+),z=(-?\d+)..(-?\d+)"
    match = re.fullmatch(pattern, line)

    is_on = match.group(1) == "on"
    xmin = int(match.group(2))
    xmax = int(match.group(3))
    ymin = int(match.group(4))
    ymax = int(match.group(5))
    zmin = int(match.group(6))
    zmax = int(match.group(7))

    bounds = BoundingBox(Range(xmin, xmax), Range(ymin, ymax), Range(zmin, zmax))

    return is_on, bounds


def read_file(filename):
    with open(filename) as f:
        return [parse_line(l.strip()) for l in f.readlines()]


def run(filename):
    cubes = []
    for is_on, bounds in read_file(filename):
        # ignore outer entries for now
        # bounds = BoundingBox.compute_overlap(bounds, CENTER)
        # if bounds.is_zero_sized:
        #     continue

        new_cube = CubeList(bounds, is_on)

        overlaps_exist = True
        while overlaps_exist:
            overlaps_exist = False
            for cube in cubes:
                if BoundingBox.compute_overlap(cube.bounds, new_cube.bounds).is_zero_sized:
                    continue

                overlaps_exist = True
                split_cubes = cube.subtract(new_cube)
                break

            if overlaps_exist:
                cubes.remove(cube)
                cubes.extend(split_cubes)
                split_cubes

        cubes.append(new_cube)

    for b1, b2 in itertools.permutations(cubes, 2):
        assert BoundingBox.compute_overlap(b1.bounds, b2.bounds).is_zero_sized

    # count those turned on
    ncubes_on = 0
    for cube in cubes:
        if cube.status:
            ncubes_on += cube.bounds.size

    print(ncubes_on)


if __name__ == "__main__":
    run(INFILE)
