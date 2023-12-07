use std::collections::HashMap;

use crate::utils::file_utils;

pub fn execute() {
    let hands = get_hands();
    let mut hands = get_classified_hands(hands);

    // sort
    for _ in 0..hands.len() {
        for i in 0..hands.len() - 1 {
            if hands[i + 1].is_better_than(&hands[i]) {
                hands.swap(i, i + 1);
            }
        }
    }

    // count points
    let mut points = 0;

    for (i, hand) in hands.iter().rev().enumerate() {
        points += (i + 1) * hand.hand.bid;
    }

    println!("{points}");
}

#[derive(Debug)]
struct Hand {
    cards: [u8; 5],
    bid: usize,
}

struct ClassifiedHand {
    hand: Hand,
    clasification: WinType,
}

impl ClassifiedHand {
    fn is_better_than(&self, other: &ClassifiedHand) -> bool {
        if self.clasification.to_points() == other.clasification.to_points() {
            for card_index in 0..self.hand.cards.len() {
                if self.hand.cards[card_index] == other.hand.cards[card_index] {
                    continue;
                } else {
                    return self.hand.cards[card_index] > other.hand.cards[card_index];
                }
            }

            return false;
        } else {
            return self.clasification.to_points() > other.clasification.to_points();
        }
    }
}

fn get_hands() -> Vec<Hand> {
    let symbol_to_number: HashMap<char, u8> = HashMap::from([
        ('T', 10),
        ('J', 11),
        ('Q', 12),
        ('K', 13),
        ('A', 14),
    ]);

    let mut hands: Vec<Hand> = vec![];

    file_utils::for_every_line("assets/input_7.txt", |line| {
        let hand_and_bid = line.split(" ").collect::<Vec<_>>();

        let mut cards: [u8; 5] = [0; 5];

        for (i, c) in hand_and_bid[0].chars().enumerate() {
            if c.is_numeric() {
                cards[i] = c.to_string().parse().unwrap();
            } else if let Some(&v) = symbol_to_number.get(&c) {
                // Convert symbol to a number so that we can easily compare them while sorting hands
                cards[i] = v;
            }
        }

        let bid: usize = hand_and_bid[1].parse().unwrap();

        hands.push(Hand { cards, bid })
    });

    hands
}

fn get_classified_hands(hands: Vec<Hand>) -> Vec<ClassifiedHand> {
    let mut hand_clasifications: Vec<ClassifiedHand> = Vec::with_capacity(hands.len());

    for hand in hands {
        let clasification =  clasify(&hand);
        hand_clasifications.push(ClassifiedHand { hand, clasification } );
    }

    hand_clasifications
}

pub enum WinType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl WinType {
    fn to_points(&self) -> usize {
        match self {
            WinType::FiveOfAKind => 7,
            WinType::FourOfAKind => 6,
            WinType::FullHouse => 5,
            WinType::ThreeOfAKind => 4,
            WinType::TwoPair => 3,
            WinType::OnePair => 2,
            WinType::HighCard => 1,
        }
    }
}

fn clasify(hand: &Hand) -> WinType {
    #[derive(Debug)]
    struct Count {
        amount: u8,
        card_id: u8,
    }

    let mut counts: Vec<Count> = vec![];

    for card_id in hand.cards {
        let mut found = false;

        for count in counts.iter_mut() {
            if card_id == count.card_id {
                found = true;
                count.amount += 1;
                break;
            }
        }

        if !found {
            counts.push(Count { amount: 1, card_id })
        }
    }

    counts.sort_by_key(|k|k.amount);

    if counts.len() == 1 {
        WinType::FiveOfAKind
    } else if counts.len() == 2 {
        if counts.last().unwrap().amount == 4 {
            WinType::FourOfAKind
        } else {
            WinType::FullHouse
        }
    } else if counts.last().unwrap().amount == 3 {
        WinType::ThreeOfAKind
    } else if counts.last().unwrap().amount == 2 {
        if counts[counts.len() - 2].amount == 2 {
            WinType::TwoPair
        } else {
            WinType::OnePair
        }
    } else {
        WinType::HighCard
    }

}
