use std::collections::HashSet;

fn main() {
    println!("Advent of Code, Day 4");
    let value: u32 = include_str!("../../examples/puzzle_input.txt").lines().map(|s| get_card_value(s.to_string())).sum();
    println!("Solution: {value}");
}

fn get_card_value(input_string: String) -> u32 {
    let game_data = input_string.split(':').nth(1).expect("There is game data").trim();
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

    let mut valid_num_count: usize = game_numbers.iter().filter(|n| winning_numbers.contains(n)).count();

    if valid_num_count == 0 {
        return 0
    }
    valid_num_count -= 1;

    2_u32.pow(valid_num_count as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input_string = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53".to_string();
        assert_eq!(get_card_value(input_string), 8);
    }
}
