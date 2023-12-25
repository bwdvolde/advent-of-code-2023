extern crate core;

use core::num;
use std::collections::HashMap;
use std::{fs};
use regex::Regex;

type NodeId = String;

#[derive(Debug)]
struct Node {
    left: NodeId,
    right: NodeId,
}

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();
    let (instructions, id_to_node) = parse(content);

    let part_1 = calculate_steps_needed(String::from("AAA"), &instructions, &id_to_node);
    println!("Part 1: {}", part_1);

    let steps_needed = id_to_node.keys()
        .map(|id| id.clone())
        .filter(|id| id.ends_with("A"))
        .map(|id| calculate_steps_needed(id, &instructions, &id_to_node))
        .collect::<Vec<_>>();
    let part_2 = lcm(&steps_needed);

    println!("Part 2: {:?}", part_2);
}

fn parse(content: String) -> (Vec<char>, HashMap<String, Node>) {
    let mut iter = content.lines().into_iter();

    let instructions: Vec<_> = iter.next().unwrap().chars().collect();
    iter.next();

    let mut id_to_node = HashMap::new();

    let regex = Regex::new(r"(?P<id>[A-Z0-9]+) = \((?P<left>[A-Z0-9]+), (?P<right>[A-Z0-9]+)\)").unwrap();

    while let Some(line) = iter.next() {
        let captures = regex.captures(line).unwrap();

        let id = String::from(&captures["id"]);
        let left = String::from(&captures["left"]);
        let right = String::from(&captures["right"]);

        id_to_node.insert(id, Node { left, right });
    }
    (instructions, id_to_node)
}

fn calculate_steps_needed(start: NodeId, instructions: &Vec<char>, id_to_node: &HashMap<String, Node>) -> usize {
    let mut steps_needed = 0;
    let mut current: NodeId = start;
    while !current.ends_with("Z") {
        let instruction = instructions[steps_needed % instructions.len()];
        current = next(id_to_node, &current, instruction);
        steps_needed += 1;
    }
    steps_needed
}

fn next(id_to_node: &HashMap<String, Node>, current: &NodeId, instruction: char) -> NodeId {
    match instruction {
        'L' => id_to_node[current].left.clone(),
        'R' => id_to_node[current].right.clone(),
        _ => panic!("Unexpected character"),
    }
}

fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}
