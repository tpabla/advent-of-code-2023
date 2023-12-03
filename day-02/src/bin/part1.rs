use std::collections::HashMap;

fn main() {

    let input = include_str!("../../examples/puzzle_input.txt");
    let possible_game = HashMap::from(
        [
            ("red", 12),
            ("green", 13),
            ("blue", 14),
        ]
    );
    println!("{}", process_list(input.to_string(), possible_game));
}


fn process_list(game_list: String, possible_game: HashMap<&str, u32>) -> u32 {
    game_list
        .lines()
        .filter(|game| is_game_possible(game, &possible_game))
        .map(|game| get_game_number(game))
        .sum()
}

fn get_game_number(game_string: &str) -> u32 {
    game_string.split(':').nth(0).unwrap_or("").replace("Game ", "").parse::<u32>().unwrap_or(0)
}

fn is_game_possible(game_input: &str, possible_game: &HashMap<&str, u32>) -> bool {
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

    for (color, num_shown) in game_map.iter() {
        match possible_game.get(color) {
            None => {
                return false;
            },
            Some(num) => {
                if num_shown > num {
                    return false;
                }
            }

        }
    }
    true
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn it_parses_the_game_number() {
        let game_string = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        assert_eq!(get_game_number(game_string), 1);
    }

    #[test]
    fn it_works_with_possible_game() {
        let game_string = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let possible_game = HashMap::from(
            [
                ("red", 12),
                ("green", 13),
                ("blue", 14),
            ]
        );
        assert_eq!(is_game_possible(game_string, &possible_game), true);
    }

    #[test]
    fn it_works_with_impossible_game() {
        let game_string = "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red";
        let possible_game = HashMap::from(
            [
                ("red", 12),
                ("green", 13),
                ("blue", 14),
            ]
        );
        assert_eq!(is_game_possible(game_string, &possible_game), false);
    }

    #[test]
    fn it_passes_the_test_input() {
        let possible_game = HashMap::from(
            [
                ("red", 12),
                ("green", 13),
                ("blue", 14),
            ]
        );
        let input_string = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(process_list(input_string.to_string(), possible_game), 8);
    }
}
