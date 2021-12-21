from dataclasses import dataclass
import itertools


# INPUT = 4, 8  # demo
INPUT = 8, 5  # actual

BOARD_SIZE = 10
MAX_SCORE = 1000


class DeterministicDice:

    def __init__(self):
        self._counter = itertools.cycle(range(1, 101))
        self.nrolls = 0

    def roll(self):
        self.nrolls += 1
        return next(self._counter)


@dataclass
class Player:

    pos: int
    score: int = 0


def make_move(player, dice):
    # breakpoint()
    move = sum(dice.roll() for _ in range(3))
    player.pos = (player.pos + move) % BOARD_SIZE
    player.score += player.pos + 1  # account for zero-based indexing


def play(player1, player2, dice):
    while True:
        make_move(player1, dice)
        if player1.score >= MAX_SCORE:
            return player1, player2

        make_move(player2, dice)
        if player2.score >= MAX_SCORE:
            return player2, player1

        # print(f"1: {player1}")
        # print(f"2: {player2}")
        # breakpoint()

if __name__ == "__main__":
    # subtract due to zero-based indexing
    pos1, pos2 = INPUT
    p1 = Player(pos1-1)
    p2 = Player(pos2-1)
    dice = DeterministicDice()

    winner, loser = play(p1, p2, dice)

    print(f"losing point count: {loser.score}")
    print(f"dice roll count: {dice.nrolls}")
    print(f"final result: {loser.score*dice.nrolls}")
