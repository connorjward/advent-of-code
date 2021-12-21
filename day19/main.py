from dataclasses import dataclass

import numpy as np


MIN_SHARED_BEACONS = 12


class Orientation:
    ...


def generate_orientations():
    for i in range(3):
        for dir in [-1, +1]:
            facing = np.zeros(3, dtype=int)
            facing[i] = dir

            up = [0, 1, 2]
            up.remove(i)

            for x in up:
                for dir_


@dataclass
class Scanner:
    
    x: int
    y: int
    z: int
    orientation: Orientation

    @property
    def default_graph(self):
        ...

    @classmethod
    def init(cls):
        graphs = {}
        for orientation in ORIENTATIONS:
            graph = set()
            for bcn1, bcn2 in permutations(self.beacons):
                offset = bcn2.coords - bcn1.coords
                graph[bcn1][bcn2] = offset
                graph[bcn2][bcn1] = -offset
            graphs[orientation] = graph



if __name__ == "__main__":
    for scanner1, scanner2 in permutations(scanners):
        for orientation in ORIENTATIONS:
            for graph in scanner2.graphs[orientation]:
                if len(scanner1.default_graph & graph) >= MIN_SHARED_BEACONS:
                    # this is it!
        # for rot in orientations:

        #     for dir in directions: 
        #         # attempt overlap
        #         # if overlap there then store rot and dir and loc
