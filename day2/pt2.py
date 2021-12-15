INFILE = "test.txt"

with open(INFILE) as f:
    lines = f.readlines()

aim = 0
xpos = 0
ypos = 0

for line in lines:
    insn, val = line.split()
    val = int(val)

    if insn == "forward":
        xpos += val
        ypos += aim * val
    elif insn == "up":
        aim -= val
    elif insn == "down":
        aim += val

print(f"xpos: {xpos}")
print(f"ypos: {ypos}")
print(f"ans: {xpos*ypos}")
