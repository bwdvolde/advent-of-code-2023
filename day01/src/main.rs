use std::fs;

fn main() {
    let content = fs::read_to_string("input.txt")
        .expect("Should have been able to read the file");

    println!("Part 1: {}", calculate_calibration_number_sum(&content));

    let mut cleaned_content = String::new();

    let mut i = 0;
    while i < content.len() {
        let mut matched_token = false;
        for token in TOKENS {
            if content[i..].starts_with(token.token) {
                cleaned_content.push_str(token.replacement);
                matched_token = true;
                break;
            }
        }
        if !matched_token {
            cleaned_content.push(content.as_bytes()[i] as char);
        }
        i += 1;
    }

    println!("Part 2: {}", calculate_calibration_number_sum(&cleaned_content))
}

struct Token<'a> {
    token: &'a str,
    replacement: &'a str,
}

const TOKENS: [Token; 9] = [
    Token {
        token: "one",
        replacement: "1",
    },
    Token {
        token: "two",
        replacement: "2",
    },
    Token {
        token: "three",
        replacement: "3",
    },
    Token {
        token: "four",
        replacement: "4",
    },
    Token {
        token: "five",
        replacement: "5",
    },
    Token {
        token: "six",
        replacement: "6",
    },
    Token {
        token: "seven",
        replacement: "7",
    },
    Token {
        token: "eight",
        replacement: "8",
    },
    Token {
        token: "nine",
        replacement: "9",
    },
];

fn calculate_calibration_number_sum(content: &String) -> i32 {
    let mut calibration_values: Vec<i32> = Vec::new();
    for line in content.lines() {
        let digits: Vec<char> = line
            .chars()
            .filter(|c| c.is_digit(10))
            .collect();

        let calibration_value: i32 = format!("{}{}", digits.first().unwrap(), digits.last().unwrap())
            .parse()
            .unwrap();
        calibration_values.push(calibration_value);
    }

    let calibration_values_sum: i32 = calibration_values.iter().sum();

    calibration_values_sum
}
