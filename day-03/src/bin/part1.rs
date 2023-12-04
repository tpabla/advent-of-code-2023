fn main() {
    println!("Advent of Code Day 3");
    println!("Solution: {}", sum_numbers(include_str!("../../examples/puzzle_input.txt").to_string()));
}


fn is_symbol(character: char) -> bool {
    !character.is_numeric() && character != '.'
}

fn sum_numbers(input_string: String) -> u32 {
    let mut nums: Vec<u32> = Default::default();

    for (i, row) in input_string.lines().into_iter().enumerate() {
        let mut prev_line = ".".repeat(row.len());
        if i > 0 {
            prev_line = input_string.lines().nth(i-1).unwrap_or(&prev_line).to_string();
        }
        let next_line = input_string.lines().nth(i+1).unwrap_or(&(".".repeat(row.len()))).to_string();

        let mut curr_num = "".to_string();
        let mut is_valid = false;
        // println!("{}", next_line);


        for (j, character) in row.chars().enumerate() {
            // println!("{}", character);
            if character.is_numeric(){
                if curr_num.is_empty() {
                    //find all things to the left
                    if j > 0 {
                        if is_symbol(row.chars().nth(j-1).unwrap_or('.')) {
                            is_valid = true;
                        }
                        if is_symbol(prev_line.chars().nth(j-1).unwrap_or('.')) {
                            is_valid = true;
                        }
                        println!("{:?}", next_line.chars().nth(j-1));

                        if is_symbol(next_line.chars().nth(j-1).unwrap_or('.')) {
                            is_valid = true;
                        }

                    }
                }
                if is_symbol(prev_line.chars().nth(j).unwrap_or('.')) {
                    is_valid = true;
                }
                if is_symbol(next_line.chars().nth(j).unwrap_or('.')) {
                    is_valid = true;
                }
                if !row.chars().nth(j+1).unwrap_or('.').is_numeric() {
                    if is_symbol(row.chars().nth(j+1).unwrap_or('.')) {
                        is_valid = true;
                    }
                    if is_symbol(prev_line.chars().nth(j+1).unwrap_or('.')) {
                        is_valid = true;
                    }
                    if is_symbol(next_line.chars().nth(j+1).unwrap_or('.')) {
                        is_valid = true;
                    }
                }
                curr_num.push(character);
            }
            if !row.chars().nth(j+1).unwrap_or('.').is_numeric(){
                if curr_num.len() > 0 && is_valid {
                    nums.push(curr_num.parse::<u32>().unwrap_or_default());
                }
                curr_num = "".to_string();
                is_valid = false;
            }
        }
    }

    println!("{:?}", nums);
    nums.iter().sum()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_with_single_row() {
        let data = "..........
467..114..
...*......";
        assert_eq!(sum_numbers(data.to_string()), 467)
    }

    #[test]
    fn it_works_with_test_input() {
        let data = include_str!("../../examples/input1.txt");
        assert_eq!(sum_numbers(data.to_string()), 4361)
    }
}
