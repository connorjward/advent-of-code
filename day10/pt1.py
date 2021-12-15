# do a depth first search of the space
# track the index


class IncompleteSyntaxError(Exception):
    ...


class CorruptedSyntaxError(Exception):

    def __init__(self, expected, found):
        super().__init__(f"Expected {expected}, but found {found} instead.")

        self.expected = expected
        self.found = found


BRACES = {"(": ")",
          "[": "]",
          "{": "}",
          "<": ">"}

BRACE_SCORES = {")": 3,
                "]": 57,
                "}": 1197,
                ">": 25137}


def match_braces(line, start_ptr=0, current_ptr=1):
    if current_ptr == len(line):
        raise IncompleteSyntaxError

    start_brace = line[start_ptr]
    next_char = line[current_ptr]

    if next_char == BRACES[start_brace]:
        # complete chunk
        return current_ptr + 1
    elif next_char in BRACES.keys():
        # new opening brace
        new_ptr = match_braces(line, current_ptr, current_ptr+1)
        return match_braces(line, start_ptr, new_ptr)
    else:
        raise CorruptedSyntaxError(BRACES[start_brace], next_char)


def read_file(filename):
    with open(filename) as f:
        return tuple(l.strip() for l in f.readlines())


if __name__ == "__main__":
    infile = "input.txt"
    lines = read_file(infile)

    score = 0

    for l in lines:
        try:
            match_braces(l)
        except CorruptedSyntaxError as ex:
            score += BRACE_SCORES[ex.found]
        except IncompleteSyntaxError:
            # ignore for now
            continue

    print(f"result: {score}")
