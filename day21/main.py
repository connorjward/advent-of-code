import collections
from dataclasses import dataclass
import itertools

import numpy as np


INPUT = 4, 8  # demo
# INPUT = 8, 5  # actual

BOARD_SIZE = 10
MAX_SCORE = 12


class DeterministicDice:

    def __init__(self):
        self._counter = itertools.cycle(range(1, 101))
        self.nrolls = 0

    def roll(self):
        self.nrolls += 1
        return next(self._counter)


def generate_dirac_rolls():
    rolls = collections.Counter()
    for a, b, c in itertools.product(range(1, 4), range(1, 4), range(1, 4)):
        rolls[a+b+c] += 1
    return rolls


DIRAC_ROLLS = generate_dirac_rolls()


@dataclass
class Player:

    pos: int
    score: int = 0

    def copy(self):
        return type(self)(self.pos, self.score)


def update_player(player, move):
    player.pos = (player.pos + move) % BOARD_SIZE
    player.score += player.pos + 1  # account for zero-based indexing


def play(player1, player2, ngames):
    if player1.score >= MAX_SCORE:
        return np.array([ngames, 0], dtype=int)
    if player2.score >= MAX_SCORE:
        return np.array([0, ngames], dtype=int)

    wins = np.zeros((2,), dtype=int)

    for move, count in DIRAC_ROLLS.items():
        p1 = player1.copy()
        p2 = player2.copy()

        update_player(p1, move)
        wins += play(p1, p2, ngames*count)

    for move, count in DIRAC_ROLLS.items():
        p1 = player1.copy()
        p2 = player2.copy()

        update_player(p2, move)
        wins += play(p1, p2, ngames*count)

    return wins


if __name__ == "__main__":
    # subtract due to zero-based indexing
    pos1, pos2 = INPUT
    p1 = Player(pos1-1)
    p2 = Player(pos2-1)

    breakpoint()
    wins = play(p1, p2, 1)

    print(f"final result: {max(wins)}")
