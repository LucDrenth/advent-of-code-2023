use crate::utils;

pub fn execute() {
    let mut total_points = 0;
    let mut cards: Vec<Card> = Vec::new();

    utils::file_utils::for_every_line("assets/input_4.txt", |line: &str| {
        cards.push(Card::from_line(line));
    });

    for card in cards {
        total_points += card.get_points();
    }

    println!("{total_points}");
}

struct Card {
    id: u32,
    winning_numbers: Vec<u32>,
    playing_numbers: Vec<u32>,
}

impl Card {
    fn from_line(line: &str) -> Self {
        let winning_and_playing_numbers = line.split("|").collect::<Vec<_>>();
        
        let card_id_and_winning_numbers = winning_and_playing_numbers[0].split(":").collect::<Vec<_>>();
        let card_id: u32 = card_id_and_winning_numbers[0].split(" ").collect::<Vec<_>>().last().unwrap().parse().unwrap();

        let mut result = Self { id: card_id, winning_numbers: Vec::new(), playing_numbers: Vec::new() };

        for winning_number in card_id_and_winning_numbers[1].split(" ").collect::<Vec<_>>() {
            if winning_number.is_empty() {
                continue;
            }

            result.winning_numbers.push(winning_number.parse().unwrap())
        }

        for playing_number in winning_and_playing_numbers[1].split(" ").collect::<Vec<_>>() {
            if playing_number.is_empty() {
                continue;
            }

            result.playing_numbers.push(playing_number.parse().unwrap())
        }

        result
    }

    fn get_points(&self) -> u32 {
        let mut points = 0;

        for playing_number in &self.playing_numbers {
            if !self.winning_numbers.contains(&playing_number) {
                continue;
            }

            if points == 0 {
                points = 1;
            } else {
                points *= 2;
            }
        }

        points
    }
}
