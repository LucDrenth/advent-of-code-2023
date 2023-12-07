use crate::utils::file_utils;

#[derive(Debug, Clone)]
struct Race {
    time: usize,
    record: usize,
}

pub fn execute() {
    let input_file_lines = file_utils::get_all_lines("assets/input_6.txt");
    let races = get_races(&input_file_lines);

    let mut ways_to_win: Vec<usize> = vec![];
    for race in races {
        ways_to_win.push(calculate_number_of_ways_to_win(&race));
    }

    let mut total = ways_to_win[0];
    for i in 1..ways_to_win.len() {
        total *= ways_to_win[i];
    }

    println!("{total}");
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

fn get_races(input: &Vec<String>) -> Vec<Race> {
    let mut races = vec![];

    let times = get_numbers(&input[0]);
    let records = get_numbers(&input[1]);

    if times.len() != records.len() {
        panic!("invalid input");
    }

    for i in 0..times.len() {
        races.push(Race { time: times[i], record: records[i] });
    }

    races
}

pub fn get_numbers(line: &String) -> Vec<usize> {
    let mut result: Vec<usize> = vec![];

    let numbers_string = line.split(":").collect::<Vec<_>>()[1];
    for number_string in numbers_string.split(" ").collect::<Vec<_>>() {
        if number_string.is_empty() {
            continue;
        }

        result.push(number_string.parse().unwrap());
    }

    result
}
