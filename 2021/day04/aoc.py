from os import environ
import numpy as np

def mark(board, val):
    board[board == val] = -1
    return np.any(np.all(board == -1, axis=0) + np.all(board == -1, axis=1))

def getSolutionPart1(numbers, boards):
    for number in numbers:
        for board in boards:
            if mark(board, number):
                return np.sum(board[board != -1]) * number

def getSolutionPart2(numbers, boards):
    for number in numbers:
        for i, board in enumerate(boards):
            if mark(board, number):
                if len(boards) == 1:
                    return np.sum(board[board != -1]) * number
                boards.pop(i)

with open('input.txt') as f:
    numbers = list(map(int, f.readline().split(',')))
    f.readline()
    boards = [np.matrix([[int(num) for num in row.split(" ") if num != ""] for row in board.split("\n")]) for board in f.read().split("\n\n")]

print((getSolutionPart2(numbers, boards) if environ.get('part') == 'part2' else getSolutionPart1(numbers, boards)))
