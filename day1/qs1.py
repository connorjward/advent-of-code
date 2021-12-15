INFILE = "input.txt"

with open(INFILE) as f:
    vals = [int(l) for l in f.readlines()]     

count = 0
for prev, curr in zip(vals, vals[1:]):
    if curr > prev:
        count += 1

print(f"Number of increases: {count}")
