use std::collections::HashMap;

fn main() {

    let input = include_str!("../../examples/puzzle_input.txt");
    println!("{}", process_list(input.to_string()));
}


fn process_list(game_list: String) -> u32 {
    game_list
        .lines()
        .map(|game| get_game_power(game))
        .sum()
}

fn get_game_power(game_input: &str) -> u32 {
    let game_string = game_input.split(':').nth(1).unwrap_or("");
    let games: Vec<&str>  = game_string.split(';').map(|s| s.trim()).collect();
    let mut game_map: HashMap<&str, u32> = Default::default();

    for game in games {
        let rounds: Vec<&str> = game.split(',').map(|s|s.trim()).collect();
        for round in &rounds {
            let mut parts = round.split_whitespace();
            let (num_shown, color) = (
                parts.next().unwrap_or_default().parse::<u32>().unwrap_or_default(),
                parts.next().unwrap_or_default()
            );

            match game_map.get(color) {
                None => {
                    game_map.insert(color, num_shown);
                },
                Some(num) => {
                    if num_shown > *num {
                        game_map.insert(color, num_shown);
                    }
                }

            }

        }
    }

    game_map.values().copied().reduce(|acc, e| acc * e).unwrap_or(0)
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn it_gets_correct_power() {
        let game_string = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        assert_eq!(get_game_power(game_string), 48);
    }

    #[test]
    fn it_passes_the_test_input() {
        let input_string = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(process_list(input_string.to_string()), 2286);
    }
}
