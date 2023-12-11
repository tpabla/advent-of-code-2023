fn main() {
    let input_string = include_str!("../../examples/puzzle_input.txt").to_string();
    println!("Advent of code, Day 9, Part 1");
    println!("Solution: {}", process_input(input_string))
}

fn get_next_value(values: Vec<i64>) -> i64 {
    let mut end_values: Vec<i64> = vec![];
    let mut collected_array: Vec<_> = values.clone();

    loop {
        collected_array = collected_array.windows(2).map(|args| args[1] -  args[0]).collect();
        end_values.push(*collected_array.last().expect("an ending number"));
        if collected_array.len() == 1 {
            break;
        }
        if collected_array.iter().filter(|&x| *x == 0).count() == collected_array.len() {
            break;
        }
    }
    end_values.iter().sum::<i64>() + values.last().expect("an ending number")
}

fn process_input(input_string: String) -> i64 {
    input_string.lines()
        .map(|s| s.split_whitespace().map(|s| s.parse::<i64>().expect("a valid int")).collect())
        .map(|input_values| get_next_value(input_values))
        .sum()
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_for_first_test_case() {
        assert_eq!(get_next_value(vec![0,3,6,9,12,15]),18)
    }

    #[test]
    fn it_works_for_second_test_case() {
        assert_eq!(get_next_value(vec![1,3,6,10,15,21]),28)
    }

    #[test]
    fn it_works_for_third_test_case() {
        assert_eq!(get_next_value(vec![10,13,16,21,30,45]),68)
    }

    #[test]
    fn it_works_with_test_input() {
        let input_string = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45".to_string();
        assert_eq!(process_input(input_string), 114);

    }

}
