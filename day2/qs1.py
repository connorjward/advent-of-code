INFILE = "input.txt"

with open(INFILE) as f:
    lines = f.readlines()

xpos = 0
ypos = 0

for line in lines:
    insn, val = line.split()
    val = int(val)

    if insn == "forward":
        xpos += val
    elif insn == "up":
        ypos -= val
    elif insn == "down":
        ypos += val

print(f"xpos: {xpos}")
print(f"ypos: {ypos}")
print(f"ans: {xpos*ypos}")
