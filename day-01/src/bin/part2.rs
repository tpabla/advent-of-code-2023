use std::collections::HashMap;

fn main() {

    let input = include_str!("../../examples/input1.txt");

    let sum: u32 = input.lines().map(|line| process(line.to_string())).sum();

    println!("Process Output: {}", sum);
}

fn reset_curr_word(num_map: &HashMap<&str, u32>, word: String) -> String {
    let mut word = word;
    while word.len() > 0 && num_map.keys().find(|key_string| key_string.starts_with(&word)) == None {
        word.remove(0);
    }
    word
}

fn process(line: String) -> u32 {
    let num_map = HashMap::from(
        [
            ("one", 1),
            ("two", 2),
            ("three", 3),
            ("four", 4),
            ("five", 5),
            ("six", 6),
            ("seven", 7),
            ("eight", 8),
            ("nine", 9),
        ]
    );

    let mut left_num: Option<u32> = None;
    let mut right_num: Option<u32> = None;
    let mut curr_word: String = "".to_string();

    for curr_char in line.chars() {
        curr_word.push(curr_char);
        if curr_char.is_numeric() {
            if left_num == None {
                left_num = Some(curr_char.to_digit(10).unwrap_or(0));
            }
            right_num = Some(curr_char.to_digit(10).unwrap_or(0));

            curr_word = reset_curr_word(&num_map, curr_word);
        }

        match num_map.get(curr_word.as_str()) {
            None => {
                match num_map.keys().find(|key_string| key_string.starts_with(&curr_word)) {
                    None => {
                        curr_word = reset_curr_word(&num_map, curr_word);
                    },
                    _ => ()
                }
            },
            Some(x) => {
                if left_num == None {
                    left_num = Some(*x);
                }
                right_num = Some(*x);
                curr_word = reset_curr_word(&num_map, curr_word);
            }
        }
    }

    return left_num.unwrap_or(0) * 10 + right_num.unwrap_or(0)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_for_number_strings() {
        assert_eq!(process("1fjdkajdfsk1".to_string()), 11);
    }

    #[test]
    fn it_works_for_text_strings() {
        assert_eq!(process("eightfjdkajdftwone".to_string()), 81);
    }

    #[test]
    fn it_works_with_one_number_string() {
        assert_eq!(process("jfdkafjdsk1fjdkafjdsk".to_string()), 11);
    }

    #[test]
    fn it_works_with_one_number_text_string() {
        assert_eq!(process("jfdkafjdskonefjdkafjdsk".to_string()), 11);
    }

    #[test]
    fn it_works_with_more_complex_string() {
        assert_eq!(process("dcnhlcqzhthreebhrnfgjbfive1threetwo53".to_string()), 33)
    }

    #[test]
    fn it_works() {
        assert_eq!(process("hbfoneight2lkrqsgtffvl".to_string()), 12)
    }
}
