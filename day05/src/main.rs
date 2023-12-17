use std::fs;

#[derive(Debug)]
struct Range {
    start: i64,
    end_inclusive: i64,
    diff: i64,
}

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();

    let (seeds, ranges_list) = parse(content);
    part_1(&seeds, &ranges_list);
}

fn part_1(seeds: &Vec<i64>, ranges_list: &Vec<Vec<Range>>) {
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

fn parse(content: String) -> (Vec<i64>, Vec<Vec<Range>>) {
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

    let mut ranges_list: Vec<Vec<Range>> = Vec::new();
    let mut ranges: Vec<Range> = Vec::new();
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
            let range = Range {
                start: source_range_start,
                end_inclusive: source_range_start + range_length,
                diff: dest_range_start - source_range_start,
            };
            ranges.push(range);
        }
    }
    ranges_list.push(ranges);
    (seeds, ranges_list)
}

fn transform(seed: i64, ranges: &Vec<Range>) -> i64 {
    for range in ranges.iter() {
        if range.start <= seed && seed <= range.end_inclusive {
            return seed + range.diff;
        }
    }
    return seed;
}
