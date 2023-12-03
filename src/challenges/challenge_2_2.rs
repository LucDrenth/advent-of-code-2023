use crate::utils;

pub fn execute() {
    let mut total: u32 = 0;

    utils::file_utils::for_every_line("assets/input_2.txt", |line: &str| {
        let result = get_min_cubes(line);
        total += result.power();
    });

    println!("{}", total);
}

struct MinCubes {
    red: u32,
    green: u32,
    blue: u32,
}

impl MinCubes {
    fn new() -> Self {
        Self { red: 0, green: 0, blue: 0 }
    }

    fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }

    fn register(&mut self, color: &str, amount: u32) {
        match color {
            "red" => {
                if self.red < amount {
                    self.red = amount;
                }
            },
            "green" => {
                if self.green < amount {
                    self.green = amount;
                }
            },
            "blue" => {
                if self.blue < amount {
                    self.blue = amount;
                }
            },
            _ => panic!("unhandled color: {color}")
        }
    }
}

/// Check if the game is possible, and if so, return the game id
fn get_min_cubes(game_data: &str) -> MinCubes {
    let game_id_and_game_result = game_data.split(":").collect::<Vec<_>>();

    let mut result = MinCubes::new();

    for grab in game_id_and_game_result[1].split(";").collect::<Vec<_>>() {
        for stone in grab.split(",").collect::<Vec<_>>() {
            let amount_and_color = stone.trim().split(" ").collect::<Vec<_>>();

            let amount: u32 = amount_and_color[0].trim().parse().unwrap();
            let color = amount_and_color[1].trim();

            result.register(color, amount);
        }
    }
        
    result
}
