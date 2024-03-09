use std::cmp::{max, min};
use std::collections::HashSet;
use std::fs;
use itertools::Itertools;

#[derive(PartialEq, Eq, Hash, Debug)]
struct Coordinate {
    r: i64,
    c: i64,
}

const GALAXY: char = '#';

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let n_cols = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..n_cols)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();

    let grid: Vec<Vec<char>> = content
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut galaxy_coordinates = HashSet::new();

    grid.iter().enumerate().for_each(|(r, row)| {
        row.iter().enumerate().for_each(|(c, char)| {
            if *char == GALAXY {
                galaxy_coordinates.insert(Coordinate { r: r as i64, c: c as i64 });
            }
        })
    });

    let rows_without_galaxies: HashSet<i64> = grid
        .iter().enumerate()
        .filter(|(_, row)| !row.contains(&'#'))
        .map(|(r, _)| r as i64)
        .collect();

    let cols_without_galaxies: HashSet<i64> = transpose(grid)
        .iter().enumerate()
        .filter(|(_, row)| !row.contains(&'#'))
        .map(|(r, _)| r as i64)
        .collect();

    let sum_shortest_paths_part_1 = calculate_sum_shortest_paths(
        &galaxy_coordinates,
        &rows_without_galaxies,
        &cols_without_galaxies,
        2,
    );
    let sum_shortest_paths_part_2 = calculate_sum_shortest_paths(
        &galaxy_coordinates,
        &rows_without_galaxies,
        &cols_without_galaxies,
        1000000,
    );

    println!("Part 1: {:?}", sum_shortest_paths_part_1);
    println!("Part 2: {:?}", sum_shortest_paths_part_2);
}

fn calculate_sum_shortest_paths(
    galaxy_coordinates: &HashSet<Coordinate>,
    rows_without_galaxies: &HashSet<i64>,
    cols_without_galaxies: &HashSet<i64>,
    cost_empty: i64,
) -> i64 {
    let sum_shortest_paths: i64 = galaxy_coordinates
        .iter()
        .tuple_combinations()
        .map(|(a, b)| {
            let manhattan_distance = (a.r - b.r).abs() + (a.c - b.c).abs();
            let n_rows_without_galaxies_between_coordinates: i64 = rows_without_galaxies
                .iter()
                .filter(|r| min(a.r, b.r) < **r && **r < max(a.r, b.r))
                .count() as i64;
            let n_cols_without_galaxies_between_coordinates: i64 = cols_without_galaxies
                .iter()
                .filter(|r| min(a.c, b.c) < **r && **r < max(a.c, b.c))
                .count() as i64;
            // - 1 because it is already present once in the manhattan distance
            manhattan_distance + (cost_empty - 1) * (n_rows_without_galaxies_between_coordinates + n_cols_without_galaxies_between_coordinates)
        })
        .sum();
    sum_shortest_paths
}
