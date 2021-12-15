from collections import defaultdict


INFILE = "input.txt"
NBITS = 12

with open(INFILE) as f:
    lines = f.readlines()

ctrs = [defaultdict(int) for _ in range(NBITS)]

for line in lines:
    for i, char in enumerate(line.strip()):
        ctrs[i][int(char)] += 1

gamma_rate = []
epsilon_rate = []
for i in range(NBITS):
    if ctrs[i][0] < ctrs[i][1]:
        gamma_rate.append("1")
        epsilon_rate.append("0")
    else:
        gamma_rate.append("0")
        epsilon_rate.append("1")

as_int = lambda rate: int("".join(rate), 2)

gamma_rate = as_int(gamma_rate)
epsilon_rate = as_int(epsilon_rate)
print(f"gamma rate: {gamma_rate}")
print(f"epsilon rate: {epsilon_rate}")
print(f"power consumption: {gamma_rate*epsilon_rate}")
