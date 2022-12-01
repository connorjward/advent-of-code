import numpy as np


FLASHED = -1
BOUNDARY = -2

NSTEPS = 100

flash_count = 0


INFILE = "puzzle.txt"


def read_file(filename):
    with open(filename) as f:
        octopi = np.array([[int(c) for c in l.strip()] for l in f.readlines()], dtype=int)

    xlen, ylen = octopi.shape

    octopi_bound = np.full((xlen+2, ylen+2), BOUNDARY, dtype=int)
    octopi_bound[1:-1, 1:-1] = octopi
    return octopi_bound


def step(octopi):
    octopi[1:-1, 1:-1] += 1

    # handle flashes and incs and repeat until no more above 9
    while (octopi > 9).any():
        for (i, j), val in np.ndenumerate(octopi):
            if val in {FLASHED, BOUNDARY}:
                continue
            if val > 9:
                octopi[i, j] = FLASHED
                global flash_count
                flash_count += 1
                inc_neighbours(octopi, i, j)

    # reset flashed to zero
    
    octopi[octopi == FLASHED] = 0


def inc_neighbours(octopi, x, y):
    for (i, j), val in np.ndenumerate(octopi[x-1:x+2, y-1:y+2]):
        # ignore already flashed octopi and boundaries
        if val in {FLASHED, BOUNDARY}:
            continue
        # is this right? depends on i, j values
        octopi[x+i-1, y+j-1] += 1



if __name__ == "__main__":
    octopi = read_file(INFILE)

    for _ in range(NSTEPS):
        step(octopi)


    print(f"result: {flash_count}")
