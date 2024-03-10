from dataclasses import dataclass
from functools import lru_cache

from read_file.read_file import read_file


@dataclass
class Entry:
    springs: str
    groups: tuple[int]


@lru_cache(maxsize=None)
def calculate_possible_arrangements(springs: str, groups: tuple[int], bomb_allowed: bool) -> int:
    if not springs:
        if not groups:
            return 1
        return 0

    result = 0

    head = springs[0]
    if head in ["#", "?"] and groups and bomb_allowed:

        i = 0
        head_group = groups[0]
        while i < len(springs) and springs[i] in ["#", "?"] and i < head_group:
            i += 1
        if i == head_group:
            result += calculate_possible_arrangements(springs[i:], groups[1:], False)
    if head in [".", "?"]:
        result += calculate_possible_arrangements(springs[1:], groups, True)

    return result


def parse():
    lines = read_file("input.txt")
    entries = []
    for line in lines:
        springs, raw_groups = line.split(" ")
        groups = tuple(int(group) for group in raw_groups.split(","))
        entries.append(Entry(springs, groups))
    return entries


entries = parse()
sum_n_possible_arrangements = sum(
    calculate_possible_arrangements(entry.springs, entry.groups, True) for entry in entries)
print(f"Part 1: {sum_n_possible_arrangements}")

amount = 5
repeated_entries = [
    Entry("?".join([entry.springs] * amount), entry.groups * amount)
    for entry in entries
]

sum_n_possible_arrangements_repeated = sum(
    calculate_possible_arrangements(entry.springs, entry.groups, True) for entry in repeated_entries)
print(f"Part 2: {sum_n_possible_arrangements_repeated}")
