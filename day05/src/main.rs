use std::fs;

use itertools::Itertools;

#[derive(Debug)]
struct RangeMap {
    start: i64,
    end_inclusive: i64,
    diff: i64,
}

#[derive(Debug)]
struct Range {
    start: i64,
    end_inclusive: i64,
}

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();

    let (seeds, maps) = parse(content);
    part_1(&seeds, &maps);

    let mut ranges: Vec<Range> = seeds
        .chunks(2)
        .map(|chunk| {
            Range {
                start: chunk[0],
                end_inclusive: chunk[0] + chunk[1],
            }
        }).collect();

    for map in maps {
        let mut split_ranges: Vec<Range> = Vec::new();
        for range in ranges {
            let mut split_points: Vec<i64> = map
                .iter()
                .flat_map(|map_range| [map_range.start, map_range.end_inclusive])
                .filter(|point| range.start < *point && *point < range.end_inclusive)
                .unique()
                .collect();
            split_points.sort();

            let mut current = range.start;
            for point in split_points {
                split_ranges.push(Range { start: current, end_inclusive: point - 1 });
                current = point;
            }
            split_ranges.push(Range { start: current, end_inclusive: range.end_inclusive });
        }

        ranges = split_ranges.iter()
            .map(|split_range| {
                Range {
                    start: transform(split_range.start, &map),
                    end_inclusive: transform(split_range.end_inclusive, &map),
                }
            })
            .collect();
    }

    let lowest_location = ranges
        .into_iter()
        .map(|range| range.start)
        .min()
        .unwrap();

    println!("Part 2: {}", lowest_location);
}

fn part_1(seeds: &Vec<i64>, ranges_list: &Vec<Vec<RangeMap>>) {
    let location_numbers: Vec<i64> = seeds
        .iter()
        .map(|seed| {
            let mut result = *seed;
            for ranges in ranges_list {
                result = transform(result, ranges);
            }
            result
        })
        .collect();

    println!("Part 1: {:?}", location_numbers.iter().min().unwrap());
}

fn parse(content: String) -> (Vec<i64>, Vec<Vec<RangeMap>>) {
    let mut iterator = content.lines();

    let seeds: Vec<i64> = iterator
        .next()
        .unwrap()
        .strip_prefix("seeds: ")
        .unwrap()
        .split(" ")
        .map(|number| number.parse().unwrap())
        .collect();


    iterator.next();
    iterator.next();

    let mut ranges_list: Vec<Vec<RangeMap>> = Vec::new();
    let mut ranges: Vec<RangeMap> = Vec::new();
    while let Some(line) = iterator.next() {
        if line.is_empty() {
            ranges_list.push(ranges);
            // Skip the header
            iterator.next();
            ranges = Vec::new();
        } else {
            let values: Vec<i64> = line
                .split(" ")
                .map(|number| number.parse().unwrap())
                .collect();
            let dest_range_start = values[0];
            let source_range_start = values[1];
            let range_length = values[2];
            let range = RangeMap {
                start: source_range_start,
                end_inclusive: source_range_start + range_length - 1,
                diff: dest_range_start - source_range_start,
            };
            ranges.push(range);
        }
    }
    ranges_list.push(ranges);
    (seeds, ranges_list)
}

fn transform(seed: i64, ranges: &Vec<RangeMap>) -> i64 {
    for range in ranges.iter() {
        if range.start <= seed && seed <= range.end_inclusive {
            return seed + range.diff;
        }
    }
    return seed;
}
