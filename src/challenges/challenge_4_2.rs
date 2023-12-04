use crate::utils;

pub fn execute() {
    let mut cards: Vec<Card> = Vec::new();

    utils::file_utils::for_every_line("assets/input_4.txt", |line: &str| {
        cards.push(Card::from_line(line));
    });

    scratch_cards(&mut cards);

    let mut number_of_cards = 0;

    for card in cards {
        number_of_cards += card.amount;
    }

    println!("{number_of_cards}");
}

fn scratch_cards(cards: &mut Vec<Card>) {
    for i in 0..cards.len() {
        let nr_winning_numbers = cards[i].get_number_of_matching_playing_numbers() as usize;

        for j in 1..=nr_winning_numbers {
            cards[i + j].amount += cards[i].amount;
        }
    }
}

struct Card {
    id: u32,
    amount: u32,
    winning_numbers: Vec<u32>,
    playing_numbers: Vec<u32>,
}

impl Card {
    fn from_line(line: &str) -> Self {
        let winning_and_playing_numbers = line.split("|").collect::<Vec<_>>();
        
        let card_id_and_winning_numbers = winning_and_playing_numbers[0].split(":").collect::<Vec<_>>();
        let card_id: u32 = card_id_and_winning_numbers[0].split(" ").collect::<Vec<_>>().last().unwrap().parse().unwrap();

        let mut result = Self { 
            id: card_id, 
            amount: 1, 
            winning_numbers: Vec::new(), 
            playing_numbers: Vec::new() 
        };

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

    fn get_number_of_matching_playing_numbers(&self) -> u32 {
        let mut matches = 0;

        for playing_number in &self.playing_numbers {
            if self.winning_numbers.contains(&playing_number) {
                matches += 1;
            }
        }

        matches
    }
}
