import numpy as np


INFILE = "input.txt"
GRID_SIZE = 5


def row_is_complete(board, idx, numbers):
    return all(val in numbers for val in board[idx, :]) 


def col_is_complete(board, idx, numbers):
    return all(val in numbers for val in board[:, idx]) 


def board_wins(board, numbers):
    nrows, ncols = board.shape
    
    row_checks = [row_is_complete(board, i, numbers) for i in range(nrows)]
    col_checks = [col_is_complete(board, i, numbers) for i in range(ncols)]
    
    return any(row_checks + col_checks)


def load_file(filename):
    top_line = np.loadtxt(filename, max_rows=1, delimiter=",", dtype=int)

    data = np.loadtxt(filename, skiprows=1, dtype=int)
    nrows, ncols = data.shape

    assert ncols == GRID_SIZE

    boards = []
    for i in range(0, nrows, GRID_SIZE):
        boards.append(data[i:i+GRID_SIZE, :])

    return top_line, tuple(boards)


if __name__ == "__main__":
    top_line, boards = load_file(INFILE)

    numbers = set()
    winning_boards = []
    for num in top_line:
        numbers.add(num)
        
        for i, board in enumerate(boards):
            # skip boards that have already won
            if i in winning_boards:
                continue

            if board_wins(board, numbers):
                winning_boards.append(i)

        if len(winning_boards) == len(boards):
            losing_board = boards[winning_boards[-1]]
            unmarked_sum = sum(val for val in losing_board.flatten()
                               if val not in numbers)
            print(f"final value: {unmarked_sum*num}")
            break
