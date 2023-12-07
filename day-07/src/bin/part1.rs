use std::{collections::HashMap, str::FromStr};
fn main() {
    let input_string = include_str!("../../examples/puzzle_input.txt").to_string();
    println!("Advent of Code: Day 7, Part 1");
    println!("Solution: {}", process(input_string));
}

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
enum Rank {
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
    Ace = 14,
}

impl Rank {
    fn max_value() -> u32 {
        Rank::Ace as u32
    }
}

impl FromStr for Rank {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "2" => Ok(Rank::Two),
            "3" => Ok(Rank::Three),
            "4" => Ok(Rank::Four),
            "5" => Ok(Rank::Five),
            "6" => Ok(Rank::Six),
            "7" => Ok(Rank::Seven),
            "8" => Ok(Rank::Eight),
            "9" => Ok(Rank::Nine),
            "T" => Ok(Rank::Ten),
            "J" => Ok(Rank::Jack),
            "Q" => Ok(Rank::Queen),
            "K" => Ok(Rank::King),
            "A" => Ok(Rank::Ace),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone)]
struct Hand {
    cards: Vec<Rank>,
    bid: u32,
}

impl Hand {
    fn get_hand_value(&self) -> u32 {
        let mut card_frequencies: HashMap<Rank, u32> = Default::default();
        for card in &self.cards {
            *card_frequencies.entry(*card).or_insert(0) += 1;
        }
        let mut sorted_frequencies: Vec<_> = card_frequencies.values().collect();
        sorted_frequencies.sort();
        sorted_frequencies
            .into_iter()
            .rev()
            .enumerate()
            .map(|(index, value)| value * (self.cards.len() as u32 - index as u32))
            .sum()
    }

    fn get_card_values(&self) -> u32 {
        let length = self.cards.len() as u32;
        self.cards
            .clone()
            .into_iter()
            .enumerate()
            .map(|(index, value)| Rank::max_value().pow(length - 1 - index as u32) * value as u32)
            .sum::<u32>()
    }
}

fn process(input_string: String) -> u32 {
    let card_lines = input_string.lines();
    let mut hands: Vec<Hand> = card_lines
        .clone()
        .into_iter()
        .map(|s| {
            let mut parts = s.split_whitespace();
            let hand: Vec<Rank> = parts
                .next()
                .expect("A valid hand")
                .chars()
                .map(|character| character.to_string().parse::<Rank>().expect("A valid rank"))
                .collect();
            let bid = parts
                .next()
                .expect("A valid vid")
                .parse::<u32>()
                .unwrap_or_default();
            Hand {
                cards: hand,
                bid: bid,
            }
        })
        .collect();

    hands.sort_by(|a, b| {
        b.get_hand_value()
            .cmp(&a.get_hand_value())
            .then_with(|| b.get_card_values().cmp(&a.get_card_values()))
    });

    hands
        .clone()
        .into_iter()
        .enumerate()
        .map(|(index, hand)| {
            let rank = hands.len() - index;
            hand.bid * rank as u32
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_rank_is_ace() {
        assert_eq!(Rank::max_value(), 14)
    }

    #[test]
    fn it_can_parse_rank_into_value() {
        assert_eq!("A".parse::<Rank>().expect("is an ace") as u32, 14)
    }

    #[test]
    fn it_works() {
        let input_string = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"
            .to_string();
        assert_eq!(process(input_string), 6440)
    }
}
