fn main() {

    let input = include_str!("../../examples/input1.txt");

    let sum: u32 = input.lines().map(|line| process(line.to_string())).sum();
    println!("Process Output: {}", sum);
}

fn process(line: String) -> u32 {

    let left_num = line.chars().find(|c| c.is_numeric()).unwrap_or('0');
    let right_num = line.chars().rev().find(|c| c.is_numeric()).unwrap_or('0');
    left_num.to_digit(10).unwrap_or(0) * 10 + right_num.to_digit(10).unwrap_or(0)
}
