from __future__ import annotations

import math
from dataclasses import dataclass
from typing import Optional

from read_file.read_file import read_file


@dataclass
class Cell:
    coordinate: Coordinate
    top: Optional[Cell]
    bottom: Optional[Cell]
    left: Optional[Cell]
    right: Optional[Cell]

    def __repr__(self):
        return repr(self.coordinate)

    def __eq__(self, other):
        return self.coordinate == other.coordinate

    def __hash__(self):
        return hash(self.coordinate)

    def neighbours(self):
        neighbours = set()
        if self.top:
            neighbours.add(self.top)
        if self.bottom:
            neighbours.add(self.bottom)
        if self.left:
            neighbours.add(self.left)
        if self.right:
            neighbours.add(self.right)
        return neighbours


@dataclass(eq=True, frozen=True)
class Coordinate:
    row: int
    col: int


lines = read_file("input.txt")
cells = {
    Coordinate(row, col): Cell(Coordinate(row, col), None, None, None, None)
    for row, _ in enumerate(lines)
    for col, _ in enumerate(lines[0])
}

s = None

for row, line in enumerate(lines):
    for col, char in enumerate(line):
        cell = cells[Coordinate(row, col)]
        match char:
            case '|':
                cell.top = cells.get(Coordinate(row - 1, col))
                cell.bottom = cells.get(Coordinate(row + 1, col))
            case '-':
                cell.left = cells.get(Coordinate(row, col - 1))
                cell.right = cells.get(Coordinate(row, col + 1))
            case 'L':
                cell.top = cells.get(Coordinate(row - 1, col))
                cell.right = cells.get(Coordinate(row, col + 1))
            case 'J':
                cell.top = cells.get(Coordinate(row - 1, col))
                cell.left = cells.get(Coordinate(row, col - 1))
            case '7':
                cell.bottom = cells.get(Coordinate(row + 1, col))
                cell.left = cells.get(Coordinate(row, col - 1))
            case 'F':
                cell.bottom = cells.get(Coordinate(row + 1, col))
                cell.right = cells.get(Coordinate(row, col + 1))
            case '.':
                continue
            case 'S':
                cell.top = cells.get(Coordinate(row - 1, col))
                cell.bottom = cells.get(Coordinate(row + 1, col))
                cell.left = cells.get(Coordinate(row, col - 1))
                cell.right = cells.get(Coordinate(row, col + 1))
                s = cell
            case _:
                raise Exception(f"Unexpected character {char}")

possible_start_coordinates = list(s.neighbours())

loop = None

for start_coordinate in possible_start_coordinates:
    previous = s
    current = start_coordinate

    coordinates_in_path = {current.coordinate}
    while current != s and len(current.neighbours() - {previous}) == 1:
        successors = current.neighbours() - {previous}
        previous = current
        current = successors.pop()
        coordinates_in_path.add(current.coordinate)

    if current == s:
        loop = coordinates_in_path
        break

part_1 = math.ceil(len(loop) / 2)
print(f"Part 1: {part_1}")

clean_grid = []
for row, line in enumerate(lines):
    clean_grid.append([])
    for col, char in enumerate(line):
        if Coordinate(row, col) in loop:
            clean_grid[row].append(char)
        else:
            clean_grid[row].append(".")

original_rows = len(lines)
original_cols = len(lines[0])

double_rows = original_rows * 2 + 1
double_cols = original_cols * 2 + 1
double_grid = [['.' for _ in range(double_cols)] for _ in range(double_rows)]

for row in range(original_rows):
    for col in range(original_cols):
        char = clean_grid[row][col]
        match char:
            case '|':
                double_grid[1 + row * 2][1 + col * 2] = '|'
                double_grid[1 + row * 2][1 + col * 2 + 1] = 'X'
                double_grid[1 + row * 2 + 1][1 + col * 2] = '|'
                double_grid[1 + row * 2 + 1][1 + col * 2 + 1] = 'X'
            case '-':
                double_grid[1 + row * 2][1 + col * 2] = '-'
                double_grid[1 + row * 2][1 + col * 2 + 1] = '-'
                double_grid[1 + row * 2 + 1][1 + col * 2] = 'X'
                double_grid[1 + row * 2 + 1][1 + col * 2 + 1] = 'X'
            case 'L':
                double_grid[1 + row * 2][1 + col * 2] = 'L'
                double_grid[1 + row * 2][1 + col * 2 + 1] = '-'
                double_grid[1 + row * 2 + 1][1 + col * 2] = 'X'
                double_grid[1 + row * 2 + 1][1 + col * 2 + 1] = 'X'
            case 'J':
                double_grid[1 + row * 2][1 + col * 2] = 'J'
                double_grid[1 + row * 2][1 + col * 2 + 1] = 'X'
                double_grid[1 + row * 2 + 1][1 + col * 2] = 'X'
                double_grid[1 + row * 2 + 1][1 + col * 2 + 1] = 'X'
            case '7':
                double_grid[1 + row * 2][1 + col * 2] = '7'
                double_grid[1 + row * 2][1 + col * 2 + 1] = 'X'
                double_grid[1 + row * 2 + 1][1 + col * 2] = '|'
                double_grid[1 + row * 2 + 1][1 + col * 2 + 1] = 'X'
            case 'F':
                double_grid[1 + row * 2][1 + col * 2] = 'F'
                double_grid[1 + row * 2][1 + col * 2 + 1] = '-'
                double_grid[1 + row * 2 + 1][1 + col * 2] = '|'
                double_grid[1 + row * 2 + 1][1 + col * 2 + 1] = 'X'
            case '.':
                continue
            case 'S':
                double_grid[1 + row * 2][1 + col * 2] = 'S'
                double_grid[1 + row * 2][1 + col * 2 + 1] = 'S'
                double_grid[1 + row * 2 + 1][1 + col * 2] = 'S'
                double_grid[1 + row * 2 + 1][1 + col * 2 + 1] = 'S'
            case _:
                raise Exception(f"Unexpected character {char}")

visited = {Coordinate(0, 0)}
stack = [Coordinate(0, 0)]
while stack:
    current = stack.pop()
    row, col = current.row, current.col

    neighbours = []
    if row > 0:
        neighbours.append(Coordinate(row - 1, col))
    if row < double_rows - 1:
        neighbours.append(Coordinate(row + 1, col))
    if col > 0:
        neighbours.append(Coordinate(row, col - 1))
    if col < double_cols - 1:
        neighbours.append(Coordinate(row, col + 1))

    for neighbour in neighbours:
        char = double_grid[neighbour.row][neighbour.col]
        if neighbour not in visited and char in ['.', "X"]:
            stack.append(neighbour)
            visited.add(neighbour)


amount_of_enclosed_cells = 0
for row, line in enumerate(double_grid):
    for col, char in enumerate(line):
        coordinate = Coordinate(row, col)
        if char == '.' and coordinate not in visited:
            amount_of_enclosed_cells += 1

amount_of_enclosed_cells //= 4

print(f"Part 2: {amount_of_enclosed_cells})
