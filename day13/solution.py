import numpy as np

from read_file.read_file import read_file

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


def calculate_score(pattern) -> int:
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


total_score = sum(calculate_score(pattern) for pattern in patterns)

print(total_score)
