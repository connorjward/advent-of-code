import numpy as np


INFILE = "input.txt"


def read_file(filename):
    """Return an array containing the positions of the crabs."""
    # Note: It may be useful to collect duplicates together for the big run
    return np.loadtxt(filename, delimiter=",", dtype=int)
    return dist


if __name__ == "__main__":
    crabs = read_file(INFILE)

    min_fuel = np.inf
    for pos in range(min(crabs), max(crabs)+1):
        fuel = sum(abs(c - pos) for c in crabs)
        if fuel < min_fuel:
            min_fuel = fuel
         
    print(f"min fuel: {min_fuel}")
