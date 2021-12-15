INFILE = "input.txt"
WINDOW_SIZE = 3

with open(INFILE) as f:
    vals = [int(l) for l in f.readlines()]     

count = 0
for i, _ in enumerate(vals):
    prev = vals[i:i+WINDOW_SIZE]
    curr = vals[i+1:i+WINDOW_SIZE+1]

    # stop when the window size begins to drop
    if len(curr) < len(prev):
        assert len(curr) == len(prev) - 1
        break

    if sum(curr) > sum(prev):
        count += 1

print(f"Number of increases: {count}")
