import sys

NDAYS = 80


def read_file(filename):
    with open(filename) as f:
        return [int(val) for val in f.readline().split(",")]


def breed(fish):
    new_fish_count = 0
    for i, f in enumerate(fish):
        if f == 0:
            fish[i] = 6
            new_fish_count += 1
        else:
            fish[i] -= 1

    for _ in range(new_fish_count):
        fish.append(8)


if __name__ == "__main__":
    infile = sys.argv[1]
    fish = read_file(infile)

    for _ in range(NDAYS):
        breed(fish)

    print(f"final fish count: {len(fish)}")
