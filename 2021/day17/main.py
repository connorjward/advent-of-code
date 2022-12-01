import numpy as np


DEMO = 20, 30, -10, -5
PUZZLE = 143, 177, -106, -71


def read_file(filename):
    ...


def find_timestep(pos, vel):
    a, b, c = -1/2, vel, -pos
    sqrt = np.sqrt(b**2 - 4*a*c)
    return (-b+sqrt)/2/a, (-b-sqrt)/2/a


def find_min_vx(xmin):
    # solve triangle number formula
    a, b, c = 1, 1, -2*xmin
    sqrt = np.sqrt(b**2 - 4*a*c)
    return np.ceil((-b+sqrt)/2/a)

def find_min_vy(bottom):
    return bottom


def find_max_vx(xmax):
    # the greatest possible velocity is to cover the distance in one step
    return xmax


def find_max_vy(bottom):
    return -bottom


def find_max_height(vy):
    y, t = 0, 0
    while vy - t > 0:
        y += vy - t
        t += 1
    return y


if __name__ == "__main__":
    myset = set()  # to store results

    xmin, xmax, bottom, top = PUZZLE

    vx = int(find_min_vx(xmin))
    while vx <= find_max_vx(xmax):
        vy = find_max_vy(bottom)
        while find_min_vy(bottom) <= vy:
            # check trajectory
            x, y, t = 0, 0, 0
            while x <= xmax:
                if t < vx:
                    x += vx - t
                y += vy - t

                if xmin <= x <= xmax and bottom <= y <= top:
                    myset.add((vx, vy))
                    break

                # catch if we undershoot
                if y < bottom and vy - t < 0:
                    break

                t += 1
            vy -= 1
        vx += 1

    # max_height = max(find_max_height(vy) for _, vy in myset)
    print(f"result: {len(myset)}")
