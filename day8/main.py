from dataclasses import dataclass
import sys


@dataclass
class Digit:

    number: int
    segments: set[str]


DISPLAY_DIGITS = {0: {"a", "b", "c", "e", "f", "g"},
                  1: {"c", "f"},
                  2: {"a", "c", "d", "e", "g"},
                  3: {"a", "c", "d", "f", "g"},
                  4: {"b", "c", "d", "f"},
                  5: {"a", "b", "d", "f", "g"},
                  6: {"a", "b", "d", "e", "f", "g"},
                  7: {"a", "c", "f"},
                  8: {"a", "b", "c", "d", "e", "f", "g"},
                  9: {"a", "b", "c", "d", "f", "g"}}


def read_file(filename):
    entries = []
    with open(filename) as f:
        for line in f.readlines():
            segmentss, output = line.strip().split("|")
            segmentss = tuple(set(c for c in segments) for segments in segmentss.split())
            entries.append((segmentss, output.split()))
    return entries


def find_possible_digits(segments, known_digits):
    poss_digits = []
    for num in DISPLAY_DIGITS.keys():
        if num in [d.number for d in known_digits]:
            continue

        # check lengths
        if len(segments) != len(DISPLAY_DIGITS[num]):
            continue

        is_poss = True
        for known_digit in known_digits:
            if DISPLAY_DIGITS[num] <= DISPLAY_DIGITS[known_digit.number] and not segments <= known_digit.segments:
                is_poss = False
            elif DISPLAY_DIGITS[num] >= DISPLAY_DIGITS[known_digit.number] and not segments >= known_digit.segments:
                is_poss = False
            else:
                pass

        if is_poss:
            poss_digits.append(Digit(num, segments))
    return poss_digits
        


def main(segmentss):
    known_digits = []
    while len(known_digits) < len(segmentss):
        for segments in segmentss:
            if segments in [d.segments for d in known_digits]:
                continue

            poss_digits = find_possible_digits(segments, known_digits)
            assert len(poss_digits) > 0
            if len(poss_digits) == 1:
                digit, = poss_digits
                known_digits.append(digit)
    return known_digits


if __name__ == "__main__":
    filename = sys.argv[1]

    entries = read_file(filename)

    final_value = 0
    for segmentss, outs in entries:
        digits = main(segmentss)

        digit_map = {frozenset(d.segments): str(d.number) for d in digits}

        final_value += int("".join([digit_map[frozenset(o)] for o in outs]))

    print(f"final value: {final_value}")
