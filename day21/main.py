"""My brute force solution is deeply horrible. I believe that there
is a clever way to do it by interleaving the routes for each player to win
but I couldn't get it to work.

I don't think matrices would work here to get linear scaling because the scoring system
is weird so one would still have an exponentially growing number of scores to
look after.
"""
import collections
from dataclasses import dataclass
import itertools

import numpy as np


# INPUT = 4, 8  # demo
INPUT = 8, 5  # actual

BOARD_SIZE = 10
MAX_SCORE = 21


gp1_wins = 0
gp2_wins = 0


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


def playtest(start, moves):
    player = Player(start)

    for move in moves:
        print(f"move: {move}")
        print(f"player: {player}")
        print()
        player.pos = (player.pos + move) % BOARD_SIZE
        player.score += player.pos + 1  # account for zero-based indexing

    print(f"player: {player}")


def mything():
    for roll in DIRAC_ROLLS:
        if complete:
            return something
        applyroll()
        recurseandchangeactiveplayer()


def generate_possible_moves_outer(player1, player2):
    for roll in DIRAC_ROLLS.keys():
        generate_possible_moves(player1, player2, [roll], 0)


def generate_possible_moves(player1, player2, moves, active):
    player1 = player1.copy()
    player2 = player2.copy()

    if active == 0:
        active_player = player1
    else:
        active_player = player2

    active_player.pos = (active_player.pos + moves[-1]) % BOARD_SIZE
    active_player.score += active_player.pos + 1  # account for zero-based indexing

    if active_player.score >= MAX_SCORE:
        if active == 0:
            global gp1_wins
            gp1_wins += calculate_dirac_score(moves)
        else:
            global gp2_wins
            gp2_wins += calculate_dirac_score(moves)
        return

    for roll in DIRAC_ROLLS.keys():
        generate_possible_moves(player1, player2, moves+[roll], (active+1)%2)


def calculate_dirac_score(wtw):
    """How many times will this get rolled?"""
    return np.prod([DIRAC_ROLLS[roll] for roll in wtw], dtype=int)


def postprocess_ways_to_win(wstw):
    counter = collections.Counter()
    for wtw in wstw:
        counter[len(wtw)] += calculate_dirac_score(wtw)
    return counter


if __name__ == "__main__":
    # subtract due to zero-based indexing
    pos1, pos2 = INPUT
    p1 = Player(pos1-1)
    p2 = Player(pos2-1)

    generate_possible_moves_outer(p1, p2)

    print(f"player1 wins: {gp1_wins}")
    print(f"player2 wins: {gp2_wins}")
