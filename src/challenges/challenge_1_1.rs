use crate::utils;

pub fn execute() {
    let mut total = 0;

    utils::file_utils::for_every_line("assets/input_1.txt", |line: &str| {
        let result = get_first_and_last_number(line);
        let result_num: u32 = result.parse().unwrap();
        total += result_num
    });

    println!("{}", total);
}

fn get_first_and_last_number(line: &str) -> String {
    let mut first: String = String::from("");
    let mut last: String = String::from("");

    for c in line.chars() {
        if c.is_numeric() {
            first = c.to_string();
            break;
        }
    }

    for c in line.chars().rev() {
        if c.is_numeric() {
            last = c.to_string();
            break;
        }
    }

    format!("{}{}", first, last)
}
