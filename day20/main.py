"""
This problem is hard because of the infinity at the edges. It is not sufficient to
just have zeros on the boundary because the outer bits can be 'flashing' and this
causes issues at the boundaries.
"""

import numpy as np


INFILE = "input.txt"
NSTEPS = 50

UNLIT_PIXEL = 0
LIT_PIXEL = 1
EDGE_PIXEL = 2


def read_file(filename):
    with open(filename) as f:
        algorithm = pixel2uint(f.readline().strip())

        # skip empty line
        f.readline()

        image = np.array([pixel2uint(line.strip()) for line in f.readlines()], dtype=np.uint8)
        image = np.pad(image, 1, constant_values=EDGE_PIXEL)
    return algorithm, image


def pixel2uint(pixels):
    return np.array([LIT_PIXEL if p == "#" else UNLIT_PIXEL for p in pixels], dtype=np.uint8)


def pixel2bin(image, i, j, edge_state):
    bin = []
    for pxl in image[i-1:i+2, j-1:j+2].flatten():
        if pxl == LIT_PIXEL:
            bin.append("1")
        elif pxl == UNLIT_PIXEL:
            bin.append("0")
        elif pxl == EDGE_PIXEL:
            bin.append(str(edge_state))
    return "".join(bin)


def bin2int(bin):
    return int(bin, 2)


def image2str(image):
    image_str = []
    for row in image:
        if all(p == EDGE_PIXEL for p in row):
            continue
        image_str.append("".join(["#" if p == LIT_PIXEL else "." for p in row if p != EDGE_PIXEL]))
    return "\n".join(image_str)


def find_boundary_state(algorithm, step):
    is_lit = UNLIT_PIXEL
    for _ in range(step):
        is_lit = algorithm[-1] if is_lit else algorithm[0]
    return is_lit


def print_image(image):
    print(image2str(image))


def process_image(image, algorithm, edge_state):
    new_image = image.copy()

    for (i, j), val in np.ndenumerate(image):
        if val == EDGE_PIXEL:
            continue

        idx = bin2int(pixel2bin(image, i, j, edge_state))

        new_image[i, j] = algorithm[idx]
    return new_image


if __name__ == "__main__":
    algorithm, image = read_file(INFILE)
    # print(image2str(image)); print()

    for i in range(NSTEPS):
        edge_state = find_boundary_state(algorithm, i)
        image = image[1:-1, 1:-1]  # strip edge pixels to insert new layer from 'infinity'
        image = np.pad(image, 1, constant_values=edge_state)
        image = np.pad(image, 1, constant_values=EDGE_PIXEL)
        # print(image2str(image)); print()

        image = process_image(image, algorithm, edge_state)
        # print(image2str(image)); print()

    # breakpoint()
    print(f"Number of lit pixels: {np.sum(image==LIT_PIXEL)}")
