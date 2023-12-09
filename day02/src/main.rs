use std::fs;
use regex::Regex;

#[derive(Debug)]
struct Game {
    id: i32,
    sets: Vec<Set>,
}

#[derive(Debug)]
struct Set {
    amount_red: i32,
    amount_blue: i32,
    amount_green: i32,
}

const MAX_RED_CUBES: i32 = 12;
const MAX_GREEN_CUBES: i32 = 13;
const MAX_BLUE_CUBES: i32 = 14;

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();

    let regex = Regex::new(r"Game (?P<game>\d+): (?P<sets>.*)").unwrap();
    let mut games: Vec<Game> = Vec::new();
    for line in content.lines() {
        let captures = regex.captures(line).unwrap();

        let id: i32 = captures["game"].parse().unwrap();
        let unparsed_sets = &captures["sets"];

        let sets = unparsed_sets
            .split("; ")
            .map(|unparsed_set| {
                let mut set = Set {
                    amount_red: 0,
                    amount_blue: 0,
                    amount_green: 0,
                };
                unparsed_set.split(", ").for_each(|cube_with_amount| {
                    let mut iter = cube_with_amount.split(" ");
                    let amount: i32 = iter.next().unwrap().parse().unwrap();
                    let name = iter.next().unwrap();
                    match name {
                        "red" => set.amount_red += amount,
                        "blue" => set.amount_blue += amount,
                        "green" => set.amount_green += amount,
                        _ => panic!("Unexpected color")
                    }
                });
                set
            }).collect();

        games.push(Game {
            id,
            sets,
        });
    }

    let sum_of_valid_game_ids: i32 = games
        .iter()
        .filter(|game| {
            game.sets.iter().all(|set| {
                set.amount_red <= MAX_RED_CUBES
                    && set.amount_blue <= MAX_BLUE_CUBES
                    && set.amount_green <= MAX_GREEN_CUBES
            })
        })
        .map(|game| game.id)
        .sum();

    println!("Part 1: {:?}", sum_of_valid_game_ids);

    let power_sum: i32 = games.iter().map(|game| {
        let max_red = game.sets.iter().map(|set| set.amount_red).max().unwrap();
        let max_blue = game.sets.iter().map(|set| set.amount_blue).max().unwrap();
        let max_green = game.sets.iter().map(|set| set.amount_green).max().unwrap();
        let power = max_red * max_blue * max_green;
        power
    }).sum();

    println!("Part 2: {:?}", power_sum);
}
