use std::collections::{HashMap, HashSet};
use std::fs;

#[derive(PartialEq, Eq, Hash, Debug)]
struct Coordinate {
    r: i32,
    c: i32,
}

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    let grid: Vec<Vec<char>> = content
        .lines()
        .map(|line| {
            // Adding a . here removes the special case that a number is the last characters of a line,
            // this makes future code less complicated
            format!("{}.", line).chars().collect()
        })
        .collect();

    let neighbours = |coordinate: Coordinate| -> Vec<Coordinate> {
        [
            Coordinate { r: coordinate.r - 1, c: coordinate.c - 1 },
            Coordinate { r: coordinate.r - 1, c: coordinate.c + 0 },
            Coordinate { r: coordinate.r - 1, c: coordinate.c + 1 },
            Coordinate { r: coordinate.r + 0, c: coordinate.c + 1 },
            Coordinate { r: coordinate.r + 1, c: coordinate.c + 1 },
            Coordinate { r: coordinate.r + 1, c: coordinate.c + 0 },
            Coordinate { r: coordinate.r + 1, c: coordinate.c - 1 },
            Coordinate { r: coordinate.r + 0, c: coordinate.c - 1 },
        ]
            .into_iter()
            .filter(|coordinate| {
                0 <= coordinate.r
                    && coordinate.r < grid.len() as i32
                    && 0 <= coordinate.c
                    && coordinate.c < grid[0].len() as i32
            })
            .collect()
    };

    let mut part_numbers = Vec::new();
    let mut gear_to_part_numbers = HashMap::new();
    for (r, _) in grid.iter().enumerate() {

        let mut is_part_number = false;
        let mut number = 0;
        let mut linked_gears = HashSet::new();

        for (c, _) in grid[r].iter().enumerate() {
            let is_number = grid[r][c].is_digit(10);
            if is_number {
                number = number * 10 + grid[r][c].to_digit(10).unwrap();
                neighbours(Coordinate { r: r as i32, c: c as i32 }).iter().for_each(|neighbour| {
                    let neighbour_char = grid[neighbour.r as usize][neighbour.c as usize];
                    if neighbour_char == '*' {
                        linked_gears.insert(Coordinate { r: neighbour.r, c: neighbour.c });
                    }
                    is_part_number |= !neighbour_char.is_digit(10) && neighbour_char != '.';
                })
            } else {
                if number > 0 && is_part_number {
                    part_numbers.push(number);
                    linked_gears.into_iter().for_each(|gear| {
                        let part_numbers_linked_to_gear = gear_to_part_numbers.entry(gear).or_insert(Vec::new());
                        (*part_numbers_linked_to_gear).push(number);
                    })
                }
                linked_gears = HashSet::new();
                number = 0;
                is_part_number = false;
            }
        }
    }

    let part_number_sum: u32 = part_numbers.iter().sum();
    println!("Part 1: {}", part_number_sum);

    let mut gear_ratio_sum: u32 = 0;

    gear_to_part_numbers.iter().for_each(|(_, part_numbers)| {
        if part_numbers.len() == 2 {
            let gear_ratio: u32 = part_numbers.into_iter().product();
            gear_ratio_sum += gear_ratio;
        }
    });
    println!("Part 2: {}", gear_ratio_sum);
}

