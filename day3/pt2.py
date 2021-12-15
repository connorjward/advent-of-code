INFILE = "input.txt"
NBITS = 12


def get_most_common(items, idx):
    zeros = 0
    ones = 0
    for item in items:
        if item[idx] == "0":
            zeros += 1
        elif item[idx] == "1":
            ones += 1
        else:
            raise AssertionError

    return "1" if zeros <= ones else "0"


def get_least_common(items, idx):
    val = get_most_common(items, idx)
    return "0" if val == "1" else "1"
    

def filter_them(items, idx, num):
    return [item for item in items if item[idx] == num]


with open(INFILE) as f:
    items = [line.strip() for line in f.readlines()]

# get oxygen rating
oxygen_items = items
for i in range(NBITS):
    if len(oxygen_items) == 1:
        break
    val = get_most_common(oxygen_items, i)
    oxygen_items = filter_them(oxygen_items, i, val)

oxygen_rating, = oxygen_items
oxygen_rating = int(oxygen_rating, 2)

# get co2 rating
co2_items = items
for i in range(NBITS):
    if len(co2_items) == 1:
        break
    val = get_least_common(co2_items, i)
    co2_items = filter_them(co2_items, i, val)

co2_rating, = co2_items
co2_rating = int(co2_rating, 2)
    
print(f"oxygen rating: {oxygen_rating}")
print(f"co2 rating: {co2_rating}")
print(f"life support rating: {oxygen_rating*co2_rating}")
