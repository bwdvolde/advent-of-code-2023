use std::fs;
use std::iter::zip;
use std::str::Lines;

#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64,
}

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();

    let races = parse(content);

    let part_1: u64 = races
        .iter()
        .map(|race| calculate_n_ways_to_beat_record(race))
        .product();

    let joined_time: u64 = races.iter()
        .map(|race| race.time.to_string())
        .collect::<Vec<String>>()
        .join("")
        .parse()
        .unwrap();

    let joined_distance: u64 = races.iter()
        .map(|race| race.distance.to_string())
        .collect::<Vec<String>>()
        .join("")
        .parse()
        .unwrap();

    let joined_race = Race { time: joined_time, distance: joined_distance };
    println!("Part 1: {:?}", part_1);
    println!("Part 2: {:?}", calculate_n_ways_to_beat_record(&joined_race));
}

fn calculate_n_ways_to_beat_record(race: &Race) -> u64 {
    let discriminant: f64 = (race.time.pow(2) - 4 * race.distance) as f64;
    let highest_root: f64 = (-(race.time as f64) - discriminant.sqrt()) / -2f64;
    let smallest_root: f64 = (-(race.time as f64) + discriminant.sqrt()) / -2f64;
    let max = highest_root.ceil() as u64 - 1;
    let min = smallest_root.floor() as u64 + 1;
    max - min + 1
}

fn parse(content: String) -> Vec<Race> {
    let mut iter = content.lines();
    let times: Vec<u64> = parse_numbers(&mut iter);
    let distances: Vec<u64> = parse_numbers(&mut iter);

    let races: Vec<Race> = zip(times, distances)
        .map(|(time, distance)| Race { time, distance })
        .collect();
    races
}

fn parse_numbers(iter: &mut Lines) -> Vec<u64> {
    iter
        .next()
        .unwrap()
        .split_whitespace()
        .collect::<Vec<&str>>()
        [1..]
        .into_iter().map(|x| x.parse().unwrap())
        .collect()
}
