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

    let lines: Vec<&str> = input_string.lines().collect();

    for (i, row) in lines.iter().enumerate() {
        let mut curr_num = "".to_string();
        let mut gear_coords: HashSet<(usize,usize)> = HashSet::new();


        for (j, character) in row.chars().enumerate() {
            if character.is_numeric(){

                let positions = [
                    (-1,-1),
                    (0,-1),
                    (1,-1),
                    (1, 0),
                    (-1, 0),
                    (1, 1),
                    (-1, 1),
                    (0, 1),
                ];
                for (row_mod, col_mod) in positions {
                    let (r, c) = (row_mod + i as i32, col_mod + j as i32);
                    if r >= 0 && r < lines.len() as i32 && c >= 0 {
                        let char = lines[r as usize].chars().nth(c as usize).unwrap_or('.');
                        if is_symbol(char) {
                            gear_coords.insert((r as usize, c as usize));
                        }
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
