use std::collections::HashMap;

use crate::utils;

pub fn execute() {
    let config: HashMap<&str, u32> = HashMap::from([
        ("red", 12),
        ("green", 13),
        ("blue", 14),
    ]);

    let mut total: u32 = 0;

    utils::file_utils::for_every_line("assets/input_2.txt", |line: &str| {
        match is_game_possible(line, &config) {
            Some(game_id) => total += game_id,
            None => (),
        }
    });

    println!("{}", total);
}

/// Check if the game is possible, and if so, return the game id
fn is_game_possible(game_data: &str, config: &HashMap<&str, u32>) -> Option<u32> {
    let game_id_and_game_result = game_data.split(":").collect::<Vec<_>>();

    let game_id: u32 = game_id_and_game_result[0][5..].parse().unwrap();

    for grab in game_id_and_game_result[1].split(";").collect::<Vec<_>>() {
        for stone in grab.split(",").collect::<Vec<_>>() {
            let amount_and_color = stone.trim().split(" ").collect::<Vec<_>>();

            let amount: u32 = amount_and_color[0].trim().parse().unwrap();
            let color = amount_and_color[1].trim();

            match config.get(color) {
                Some(max) => {
                    if amount > *max {
                        // println!("game {} is impossible due to {} {}", game_id, amount, color);
                        return None;
                    }
                },
                None => panic!("unhandled color: {}", color),
            }
        }
    }
        
    Some(game_id)
}
