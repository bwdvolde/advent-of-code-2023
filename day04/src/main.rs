use std::collections::{HashMap, HashSet};
use std::fs;
use regex::Regex;

#[derive(Debug)]
struct Card {
    id: u32,
    winning_numbers: HashSet<u32>,
    numbers: Vec<u32>,
}

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    let cards = parse(&content);
    part_1(&cards);
    part_2(&cards);
}

fn parse(content: &str) -> Vec<Card> {
    let regex = Regex::new(r"Card +(?P<id>\d+): (?P<winning_cards>[^|]+) \| (?P<cards>[^|]+)").unwrap();

    content.lines().map(|line| {
        let captures = regex.captures(line).unwrap();

        let id: u32 = captures["id"].parse().unwrap();

        let unparsed_winning_numbers = &captures["winning_cards"];
        let winning_numbers: HashSet<u32> = unparsed_winning_numbers.split_whitespace()
            .map(|number| number.parse().unwrap())
            .collect();

        let unparsed_numbers = &captures["cards"];
        let numbers: Vec<u32> = unparsed_numbers.split_whitespace()
            .map(|number| {
                number.parse().unwrap()
            })
            .collect();

        Card { id, winning_numbers, numbers }
    }).collect()
}

fn part_1(cards: &Vec<Card>) {
    let points_sum: u32 = cards.iter().map(|card| {
        let n_winning_numbers = card.numbers.iter().filter(|number| {
            card.winning_numbers.contains(number)
        }).count();
        if n_winning_numbers == 0 {
            0
        } else {
            2u32.pow((n_winning_numbers - 1) as u32)
        }
    }).sum();

    println!("Part 1: {}", points_sum);
}

fn part_2(cards: &Vec<Card>) {
    let mut card_to_n_copies = HashMap::new();
    cards.iter().for_each(|card| {
        card_to_n_copies.insert(card.id, 1);
    });

    cards.iter().for_each(|card| {
        let n_winning_numbers = card.numbers.iter().filter(|number| {
            card.winning_numbers.contains(number)
        }).count();

        let n_copies_card = card_to_n_copies.get(&card.id).unwrap().clone();
        (1..=n_winning_numbers).for_each(|i| {
            let card_id_to_add_copies_to = card.id + (i as u32);
            if let Some(n_copies) = card_to_n_copies.get_mut(&card_id_to_add_copies_to) {
                *n_copies += n_copies_card;
            }
        })
    });

    let total_cards: u32 = card_to_n_copies.values().sum();

    println!("Part 2: {}", total_cards);
}

