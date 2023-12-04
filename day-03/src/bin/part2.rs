use std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    println!("Advent of Code Day 3");
    println!("Solution: {}", sum_numbers(include_str!("../../examples/puzzle_input.txt").to_string()));
}


fn is_symbol(character: char) -> bool {
    character == '*'
}

fn sum_numbers(input_string: String) -> u32 {
    let mut gear_map: HashMap<String, Vec<u32>> = Default::default();

    for (i, row) in input_string.lines().into_iter().enumerate() {
        let mut prev_line = ".".repeat(row.len());
        if i > 0 {
            prev_line = input_string.lines().nth(i-1).unwrap_or(&prev_line).to_string();
        }
        let next_line = input_string.lines().nth(i+1).unwrap_or(&(".".repeat(row.len()))).to_string();

        let mut curr_num = "".to_string();
        let mut gear_coords: HashSet<(usize,usize)> = HashSet::new();


        for (j, character) in row.chars().enumerate() {
            if character.is_numeric(){
                if curr_num.is_empty() {
                    //find all things to the left
                    if j > 0 {
                        if is_symbol(row.chars().nth(j-1).unwrap_or('.')) {
                            gear_coords.insert((i, j-1));
                        }
                        if is_symbol(prev_line.chars().nth(j-1).unwrap_or('.')) {
                            gear_coords.insert((i-1, j-1));
                        }

                        if is_symbol(next_line.chars().nth(j-1).unwrap_or('.')) {
                            gear_coords.insert((i+1, j-1));
                        }

                    }
                }
                if is_symbol(prev_line.chars().nth(j).unwrap_or('.')) {
                    gear_coords.insert((i-1, j));
                }
                if is_symbol(next_line.chars().nth(j).unwrap_or('.')) {
                    gear_coords.insert((i+1, j));
                }
                if !row.chars().nth(j+1).unwrap_or('.').is_numeric() {
                    if is_symbol(row.chars().nth(j+1).unwrap_or('.')) {
                        gear_coords.insert((i, j+1));
                    }
                    if is_symbol(prev_line.chars().nth(j+1).unwrap_or('.')) {
                        gear_coords.insert((i-1, j+1));
                    }
                    if is_symbol(next_line.chars().nth(j+1).unwrap_or('.')) {
                        gear_coords.insert((i+1, j+1));
                    }
                }
                curr_num.push(character);
            }
            if !row.chars().nth(j+1).unwrap_or('.').is_numeric(){
                if curr_num.len() > 0 && gear_coords.len() > 0 {
                    for coord in &gear_coords {
                        let coord_string: String = format!("{:?}", coord);
                        match gear_map.get(&coord_string) {
                            None => {
                                gear_map.insert(coord_string, Vec::from([curr_num.parse::<u32>().unwrap()]));
                            },
                            Some(x) => {
                                let mut curr_nums = x.clone();
                                curr_nums.push(curr_num.parse::<u32>().unwrap());
                                gear_map.insert(coord_string, curr_nums);
                            }
                        }
                    }
                }
                curr_num = "".to_string();
                gear_coords = HashSet::new()
            }
        }
    }

    let mut product_output: Vec<u32> = Vec::new();
    for (_coord, nums) in gear_map.iter() {
        if nums.len() == 2 {
            product_output.push(nums.iter().product())
        }
    }

    product_output.iter().sum()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_with_test_input() {
        let data = include_str!("../../examples/input1.txt");
        assert_eq!(sum_numbers(data.to_string()), 467835)
    }
}
