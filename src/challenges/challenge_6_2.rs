use crate::utils::file_utils;

#[derive(Debug, Clone)]
struct Race {
    time: usize,
    record: usize,
}

pub fn execute() {
    let input_file_lines = file_utils::get_all_lines("assets/input_6.txt");
    let race = get_race(&input_file_lines);
    
    println!("{}", calculate_number_of_ways_to_win(&race));
}

fn calculate_number_of_ways_to_win(race: &Race) -> usize {
    let mut ways_to_win = 0;

    for i in 1..race.time {
        let distance = i * (race.time - i);

        if distance > race.record {
            ways_to_win += 1;
        }
    }

    ways_to_win
}

fn get_race(input: &Vec<String>) -> Race {
    Race {
        time: get_number(&input[0]),
        record: get_number(&input[1]),
    }
}

pub fn get_number(line: &String) -> usize {
    let numbers_string = line.split(":").collect::<Vec<_>>()[1];

    let mut buffer: String = "".to_string();

    for number_string in numbers_string.split(" ").collect::<Vec<_>>() {
        if number_string.is_empty() {
            continue;
        }

        buffer = format!("{buffer}{number_string}");
    }

    buffer.parse().unwrap()
}
