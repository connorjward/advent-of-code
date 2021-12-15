# do a depth first search of the space
# track the index


class IncompleteSyntaxError(Exception):

    def __init__(self, brace, trace=None):
        super().__init__(f"stuff broke...")

        self.trace = (brace,) if trace is None else trace + (brace,)


class CorruptedSyntaxError(Exception):

    def __init__(self, expected, found):
        super().__init__(f"Expected {expected}, but found {found} instead.")

        self.expected = expected
        self.found = found


BRACES = {"(": ")",
          "[": "]",
          "{": "}",
          "<": ">"}

BRACE_SCORES = {")": 1,
                "]": 2,
                "}": 3,
                ">": 4}


def match_braces(line, start_ptr=0, current_ptr=1):
    try:
        start_brace = line[start_ptr]
        next_char = line[current_ptr]
    except IndexError:
        raise IncompleteSyntaxError(BRACES[start_brace])


    if next_char == BRACES[start_brace]:
        # complete chunk
        return current_ptr + 1
    elif next_char in BRACES.keys():
        # new opening brace
        try:
            new_ptr = match_braces(line, current_ptr, current_ptr+1)
        except IncompleteSyntaxError as ex:
            raise IncompleteSyntaxError(BRACES[start_brace], ex.trace)

        return match_braces(line, start_ptr, new_ptr)
    else:
        raise CorruptedSyntaxError(BRACES[start_brace], next_char)


def read_file(filename):
    with open(filename) as f:
        return tuple(l.strip() for l in f.readlines())


def calculate_score(trace):
    score = 0

    for brace in trace:
        score *= 5
        score += BRACE_SCORES[brace]

    return score


if __name__ == "__main__":
    infile = "input.txt"
    lines = read_file(infile)

    scores = []

    for l in lines:
        try:
            start, curr = 0, 1
            while curr < len(l):
                val = match_braces(l, start, curr)
                start, curr = curr+1, val+1
        except CorruptedSyntaxError as ex:
            ...
        except IncompleteSyntaxError as ex:
            scores.append(calculate_score(ex.trace))

    scores.sort()
    mid_score_idx = len(scores) // 2

    print(f"result: {scores[mid_score_idx]}")
