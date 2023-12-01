use std::collections::HashMap;

use crate::utils;

pub fn execute() {
    let string_numbers: HashMap<String, u32> = HashMap::from([
        ("one".to_string(), 1),
        ("two".to_string(), 2),
        ("three".to_string(), 3),
        ("four".to_string(), 4),
        ("five".to_string(), 5),
        ("six".to_string(), 6),
        ("seven".to_string(), 7),
        ("eight".to_string(), 8),
        ("nine".to_string(), 9),
    ]);





    let mut total = 0;

    utils::file_utils::for_every_line("assets/input_1.txt", |line: &str| {
        let first = get_first_number(line, &string_numbers);
        let last = get_last_number(line, &string_numbers);

        let combined: u32 = format!("{}{}", first, last).parse().unwrap();
        total += combined;
    });

    println!("{}", total);


    // println!("{}", get_last_number("2733vmmpknvgr", &string_numbers));
}

fn get_first_number(line: &str, string_numbers: &HashMap<String, u32>) -> u32 {
    for (i, c) in line.chars().enumerate() {
        if c.is_numeric() {
            let result: u32 = c.to_string().parse().unwrap();
            return result;
        }

        for j in 0..=i {
            let str_to_match = &line[j..=i];

            match string_numbers.get(str_to_match) {
                Some(v) => return *v,
                None => (),
            }
        }
    }

    panic!("no nunber found in line {}", line)
}

fn get_last_number(line: &str, string_numbers: &HashMap<String, u32>) -> u32 {
    for (i, c) in line.chars().rev().enumerate() {
        if c.is_numeric() {
            let result: u32 = c.to_string().parse().unwrap();
            return result;
        }

        for j in (line.len() - i - 1)..line.len() {
            let str_to_match = &line[(line.len() - i - 1)..=j];

            match string_numbers.get(str_to_match) {
                Some(v) => return *v,
                None => (),
            }
        }
    }

    panic!("no nunber found in line {}", line)
}
