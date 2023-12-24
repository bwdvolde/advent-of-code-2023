use std::cmp::Ordering;
use std::fmt::{Debug, Formatter, Write};
use std::fs;

use counter::Counter;

#[derive(Hash, PartialEq, Eq, Ord)]
struct Card {
    char: char,
}

const JOKER: Card = Card { char: 'J' };

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let characters = ['A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'J'];
        let index_self = characters.iter().position(|c| *c == self.char).unwrap();
        let index_other = characters.iter().position(|c| *c == other.char).unwrap();
        return index_other.partial_cmp(&index_self);
    }
}

impl Debug for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_char(self.char)
    }
}

#[derive(Ord, Eq, PartialEq)]
struct Hand {
    cards: [Card; 5],
    bid: u32,
}

impl PartialOrd<Self> for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.hand_type().partial_cmp(&other.hand_type())
            .map(|ordering| {
                match ordering {
                    Ordering::Less => ordering,
                    Ordering::Greater => ordering,
                    Ordering::Equal => self.cards.partial_cmp(&other.cards).unwrap(),
                }
            })
    }
}

impl Debug for Hand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_list()
            .entries(&self.cards)
            .finish()
    }
}

impl Hand {
    fn hand_type(&self) -> HandType {
        let counts = self.cards.iter().collect::<Counter<_>>();
        let amount_of_a_kind = counts
            .values()
            .collect::<Counter<_>>();


        let counts_no_joker = self
            .cards
            .iter()
            .filter(|card| **card != JOKER)
            .collect::<Counter<_>>();
        let amount_of_a_kind_no_joker = counts_no_joker
            .values()
            .collect::<Counter<_>>();

        let highest_kind_no_joker = **(amount_of_a_kind_no_joker.keys().max().unwrap_or(&&0)) as u32;
        let joker_count = counts.get(&JOKER).unwrap_or(&0).clone() as u32;

        if highest_kind_no_joker + joker_count == 5 {
            return HandType::FiveOfAKind;
        }
        if highest_kind_no_joker + joker_count == 4 {
            return HandType::FourOfAKind;
        }
        // Don't have to check if you can make full house with more than one JOKER because if you can, you can just make a four/five of a kind
        if (amount_of_a_kind.contains_key(&3) && amount_of_a_kind.contains_key(&2))
            || (amount_of_a_kind_no_joker.get(&2) == Some(&2) && joker_count == 1) {
            return HandType::FullHouse;
        }
        if highest_kind_no_joker + joker_count == 3 {
            return HandType::ThreeOfAKind;
        }
        // Don't have to check if you can get two pairs with a JOKER here because if you have a JOKER can make a three of a kind
        if amount_of_a_kind.get(&2) == Some(&2) {
            return HandType::TwoPair;
        }
        if highest_kind_no_joker + joker_count == 2 {
            return HandType::OnePair;
        }
        HandType::HighCard
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();

    let mut hands = content.lines().map(|line| {
        let mut iter = line.split(" ");
        let cards: [Card; 5] = iter.next().unwrap()
            .chars()
            .into_iter()
            .map(|char| Card { char })
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let bid: u32 = iter.next().unwrap().parse().unwrap();
        Hand { cards, bid }
    })
        .collect::<Vec<_>>();

    hands.sort();
    let total_winnings: u32 = hands
        .iter()
        .enumerate()
        .map(|(i, hand)| {
            (i as u32 + 1) * hand.bid
        }).sum();
    println!("Part 2: {:?}", total_winnings);
}
