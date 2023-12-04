use std::collections::{HashSet, HashMap};

fn main() {
    println!("Advent of Code, Day 4");
    let value: u32 = how_many_scratchcards(include_str!("../../examples/puzzle_input.txt").to_string());
    println!("Solution: {value}");
}

fn how_many_scratchcards(input_string: String) -> u32 {
    let mut card_map: HashMap<u32, u32> = Default::default();

    for line in input_string.lines() {
        let (card_number, value) = get_card_value(line.to_string());
        *card_map.entry(card_number).or_insert(0) += 1;

        let num_cards: &u32 = card_map.get(&card_number).unwrap_or(&0);
        for _card in 0..*num_cards{
            for num in (card_number + 1)..(card_number + value + 1) {
                *card_map.entry(num).or_insert(0) += 1;
            }
        }
    }
    card_map.values().sum()
}

fn get_card_value(input_string: String) -> (u32, u32) {
    let (card_value, game_data) = {
        let mut parts = input_string.split(":");
        (
            parts.next()
                .expect("Card Number String")
                .trim()
                .trim_start_matches("Card ")
                .trim().parse::<u32>()
                .unwrap_or_default(),
            parts.next().expect("Game data"),
        )
    };
    let (game_numbers, winning_numbers): (HashSet<u32>, HashSet<u32>) =  {
        let mut parts = game_data.split('|');
        (
            parts.next().expect("valid game numbers").trim()
                .split(' ')
                .map(|s| s.trim().parse::<u32>().unwrap_or_default())
                .filter(|s| *s as u32 != 0)
                .collect(),
            parts.next().expect("valid winning numbers").trim()
                .split(' ')
                .map(|s| s.trim().parse::<u32>().unwrap_or_default())
                .filter(|s| *s as u32 != 0)
                .collect()
        )
    };

    (
        card_value,
        game_numbers.iter()
            .filter(|n| winning_numbers.contains(n))
            .count() as u32
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_on_a_single_line() {
        let input_string = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53".to_string();
        assert_eq!(get_card_value(input_string), (1, 4));
    }

    #[test]
    fn it_works_with_test_input() {
        let input_string = include_str!("../../examples/part1_input.txt").to_string();
        assert_eq!(how_many_scratchcards(input_string), 30);
    }
}
