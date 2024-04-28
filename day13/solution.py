from dataclasses import dataclass

import numpy as np

from read_file.read_file import read_file


@dataclass(eq=True, frozen=True)
class Coordinate:
    row: int
    col: int


lines = read_file("input.txt")

patterns = []

iterator = iter(lines)

while True:
    pattern = []
    try:
        while line := next(iterator):
            pattern.append(list(line))
        patterns.append(pattern)
    except StopIteration:
        patterns.append(pattern)
        break

patterns = [np.array(pattern) for pattern in patterns]


def calculate_score_part_1(pattern) -> int:
    score = 0

    n_columns = len(pattern[0])
    for i in range(n_columns):
        j = 0
        while i + j < n_columns and i - 1 - j >= 0 and (pattern[:, i + j] == pattern[:, i - 1 - j]).all():
            j += 1
        is_reflection = not (i + j < n_columns and i - 1 - j >= 0)
        if is_reflection:
            score += i

    n_rows = len(pattern)
    for i in range(n_rows):
        j = 0
        while i + j < n_rows and i - 1 - j >= 0 and (pattern[i + j,] == pattern[i - 1 - j]).all():
            j += 1
        is_reflection = not (i + j < n_rows and i - 1 - j >= 0)
        if is_reflection:
            score += 100 * i

    return score


def calculate_score_part_2(pattern) -> int:
    # In fact there are always 2 smudges, because if you find one smudge the one it mirrors can also be a smudge.
    # This means that the original reflection point breaks, because you can choose one out of the 2 smudges and the
    # assignment doesn't specify which one.
    n_columns = len(pattern[0])
    for i in range(n_columns):
        j = 0
        n_smudges = 0
        while i + j < n_columns and i - 1 - j >= 0 and n_smudges <= 1:
            n_smudges += np.sum(pattern[:, i + j] != pattern[:, i - 1 - j])
            j += 1

        if n_smudges == 1:
            return i

    n_rows = len(pattern)
    for i in range(n_rows):
        j = 0
        n_smudges = 0
        while i + j < n_rows and i - 1 - j >= 0 and n_smudges <= 1:
            n_smudges += np.sum(pattern[i + j, :] != pattern[i - 1 - j, :])
            j += 1

        if n_smudges == 1:
            return i * 100


total_score_part_1 = sum(calculate_score_part_1(pattern) for pattern in patterns)
total_score_part_2 = sum(calculate_score_part_2(pattern) for pattern in patterns)

print(f"Part 1: {total_score_part_1}")
print(f"Part 2: {total_score_part_2}")
