use itertools::Itertools;
use std::fs;

struct PasswordData {
    password: String,
    letter_to_compare: char,
    values_to_match: (usize, usize),
}

impl PasswordData {
    fn new(line: String) -> PasswordData {
        let mut parts = line.split_whitespace();

        let values_to_match = parts
            .next()
            .unwrap()
            .split("-")
            .collect::<Vec<&str>>()
            .iter()
            .map(|s| s.parse::<usize>().unwrap())
            .next_tuple()
            .unwrap();

        let letter_to_compare = parts
            .next()
            .unwrap()
            .replace(":", "")
            .parse::<char>()
            .unwrap();

        let password = String::from(parts.next().unwrap());

        PasswordData {
            password,
            letter_to_compare,
            values_to_match,
        }
    }
}

pub fn main() {
    let contents =
        fs::read_to_string("./input/02.txt").expect("Something went wrong reading the file");

    let mut total = 0;

    for line in contents.lines() {
        let password_data = PasswordData::new(String::from(line));

        let (min, max) = password_data.values_to_match;

        let count = password_data
            .password
            .matches(password_data.letter_to_compare)
            .count();

        if count >= min && count <= max {
            total += 1;
        }
    }

    println!("total: {}", total);

    let mut total = 0;

    for line in contents.lines() {
        let password_data = PasswordData::new(String::from(line));

        let (valid, invalid) = password_data.values_to_match;

        let password_chars: Vec<char> = password_data.password.chars().collect();

        let pos_1 = password_chars[valid - 1];
        let pos_2 = password_chars[invalid - 1];

        if (pos_1 == password_data.letter_to_compare) != (pos_2 == password_data.letter_to_compare)
        {
            total += 1;
        }
    }

    println!("total: {}", total);
}
