use std::fs;

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();

    let histories = content
        .lines().map(|line| {
        line
            .split_whitespace()
            .into_iter()
            .map(|number| number.parse::<i64>().unwrap())
            .collect::<Vec<_>>()
    }).collect::<Vec<_>>();

    let sum_of_next_values: i64 = histories
        .iter()
        .map(|history| predict_next_value(history.clone()))
        .sum();

    println!("Part 1: {}", sum_of_next_values);

    let sum_of_previous_values: i64 = histories
        .iter()
        .map(|history| predict_previous_value(history.clone()))
        .sum();

    println!("Part 2: {}", sum_of_previous_values);

}

fn predict_next_value(history: Vec<i64>) -> i64 {
    let differences = calculate_differences(history);

    let mut i = differences.len() - 1;
    let mut current = 0;
    while i > 0 {
        i = i - 1;
        current = differences[i].last().unwrap() + current;
    }
    current
}

fn predict_previous_value(history: Vec<i64>) -> i64 {
    let differences = calculate_differences(history);

    let mut i = differences.len() - 1;
    let mut current = 0;
    while i > 0 {
        i = i - 1;
        current = differences[i].first().unwrap() - current;
    }
    current
}

fn calculate_differences(history: Vec<i64>) -> Vec<Vec<i64>> {
    let mut differences: Vec<Vec<i64>> = vec!(history);
    while !differences.last().unwrap().iter().all(|number| *number == 0) {
        let new_differences = differences.last().unwrap()
            .windows(2)
            .map(|pair| pair[1] - pair[0])
            .collect();
        differences.push(new_differences)
    };
    differences
}
