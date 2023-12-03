use crate::utils;

#[derive(Debug, Clone)]
struct Number {
    line_index: usize,
    string: String,
    start_char_index: usize,
}

pub fn execute() {
    let mut total: u32 = 0;

    let lines = utils::file_utils::get_all_lines("assets/input_3.txt");

    let numbers = get_numbers(&lines);
    let numbers = get_numbers_with_adjacent_symbol(numbers, &lines);

    for number in numbers {
        let value: u32 = number.string.parse().unwrap();
        total += value;
    }

    println!("{}", total);
}

fn get_numbers(lines: &Vec<String>) -> Vec<Number> {
    let mut numbers: Vec<Number> = Vec::new();

    for (line_index, line) in lines.iter().enumerate() {

        let mut start_current_number: Option<usize> = None;
        let mut number_buffer: String = "".to_string();

        for (char_index, c) in line.chars().enumerate() {
            if c.is_numeric() {
                if number_buffer.is_empty() {
                    start_current_number = Some(char_index);
                }

                number_buffer.push(c);
            } else {
                if !number_buffer.is_empty() {
                    numbers.push(Number { 
                        line_index: line_index, 
                        string: number_buffer.clone(), 
                        start_char_index: start_current_number.unwrap(),
                    })
                }

                start_current_number = None;
                number_buffer = "".to_string();
            }
        }

        if !number_buffer.is_empty() {
            numbers.push(Number { 
                line_index, 
                string: number_buffer.clone(), 
                start_char_index: start_current_number.unwrap(),
            })
        }
    }

    numbers
}

fn get_numbers_with_adjacent_symbol(numbers: Vec<Number>, lines: &Vec<String>) -> Vec<Number> {
    let mut result: Vec<Number> = Vec::new();

    for number in numbers {
        let mut has_symbol_neighbour = false;

        for index_to_check in number.start_char_index..(number.start_char_index + number.string.len()) {
            for x in -1..=1 {
                for y in -1..=1 {
                    // no need to check the number for which we look for symbol neighbours
                    if x == 0 && y == 0 {
                        continue;
                    }

                    let neighbour_line_index = number.line_index as i32 + y;
                    let neighhbour_index = index_to_check as i32 + x;

                    // check if the neighbour line exists
                    if neighbour_line_index < 0 || neighbour_line_index >= lines.len() as i32 {
                        continue;
                    }

                    let neighbour_line = &lines[neighbour_line_index as usize];

                    // check if the neighbour char exists
                    if neighhbour_index < 0 || neighhbour_index >= neighbour_line.len() as i32 {
                        continue;
                    }

                    let neighbour_char = neighbour_line.chars().nth(neighhbour_index as usize).unwrap();

                    if !neighbour_char.eq(&'.') && !neighbour_char.is_numeric() {
                        has_symbol_neighbour = true;
                    }
                }
            }
        }

        if has_symbol_neighbour {
            result.push(number.clone())
        }
    }

    result
}
