use crate::utils;

#[derive(Debug, Clone)]
struct Number {
    line_index: usize,
    string: String,
    start_char_index: usize,
}

#[derive(Debug)]
struct Gear {
    adjacent: [String; 2],
}

pub fn execute() {
    let mut total: u32 = 0;

    let lines = utils::file_utils::get_all_lines("assets/input_3.txt");

    let numbers = get_numbers(&lines);
    let gears = get_gears(&numbers, &lines);

    for gear in gears {
        let num_1: u32 = gear.adjacent[0].parse().unwrap();
        let num_2: u32 = gear.adjacent[1].parse().unwrap();
        let gear_ratio = num_1 * num_2;
        total += gear_ratio;
    }

    println!("{total}");
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

fn get_gears(numbers: &Vec<Number>, lines: &Vec<String>) -> Vec<Gear> {
    let mut result = Vec::new();

    for (line_index, line) in lines.iter().enumerate() {
        for (index_to_check, char) in line.chars().enumerate() {
            if char != '*' {
                continue;
            }

            let mut first_neighbour_num: Option<String> = None;
            let mut second_neighbour_num: Option<String> = None;

            for y in -1..=1 {
                let mut x = -1;
                while x <= 1 {
                    // no need to check the number for which we look for symbol neighbours
                    if x == 0 && y == 0 {
                        x += 1;
                        continue;
                    }

                    let neighbour_line_index = line_index as i32 + y;
                    let neighhbour_index = index_to_check as i32 + x;

                    // check if the neighbour line exists
                    if neighbour_line_index < 0 || neighbour_line_index >= lines.len() as i32 {
                        break;
                    }

                    let neighbour_line = &lines[neighbour_line_index as usize];

                    // check if the neighbour char exists
                    if neighhbour_index < 0 || neighhbour_index >= neighbour_line.len() as i32 {
                        x += 1;
                        continue;
                    }

                    let neighbour_char = neighbour_line.chars().nth(neighhbour_index as usize).unwrap();

                    if neighbour_char.is_numeric() {
                        // find the number in our numbers list
                        for num in numbers {
                            if num.line_index == neighbour_line_index as usize && (neighhbour_index as usize) >= num.start_char_index && (neighhbour_index as usize) < num.start_char_index + num.string.len() {
                                // register as neigbour
                                if first_neighbour_num.is_none() {
                                    first_neighbour_num = Some(num.string.clone());
                                } else if second_neighbour_num.is_none() {
                                    second_neighbour_num = Some(num.string.clone());
                                } else {
                                    panic!("more than 2 neigbour numbers!")
                                }

                                x = num.start_char_index as i32 + num.string.len() as i32 - index_to_check as i32;
                                break;
                            }
                        }
                    }

                    x += 1;
                }
            }

            if first_neighbour_num.is_some() && second_neighbour_num.is_some() {
                result.push(Gear { 
                    adjacent: [first_neighbour_num.unwrap(), second_neighbour_num.unwrap()] 
                });
            }
        }
    }

    result
}
