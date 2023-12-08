use std::collections::HashMap;

fn main() {
    let input_string = include_str!("../../examples/puzzle_input.txt").to_string();
    println!("Advent of Code: Day 7, Part 1");
    println!("Solution: {}", process(input_string));
}


struct Node {
    current: String,
    left: String,
    right: String
}

fn process(input_string: String) -> u32 {
    let mut parts = input_string.split("\n\n");
    let directions = parts.next().expect("a valid string");
    let camel_map = parts.next().expect("a valid string");

    let mut map_hash: HashMap<String, Node> = Default::default();

    let mut count: u32 = 0;
    for map_line in camel_map.lines() {
        let mut parts = map_line.split('=');
        let node = parts
            .next()
            .expect("a valid string")
            .trim().to_string();
        let node_directions: Vec<String> = parts
            .next()
            .expect("a valid string")
            .trim()
            .trim_matches(|c| {
                ['(', ')'].contains(&c)
            }).split(',')
            .map(|s| s.trim().to_string())
            .collect();
        if node_directions.len() != 2 {
            panic!("Invalid left and right directions in input")
        }

        map_hash.entry(node.clone()).or_insert(Node{
            current: node,
            left: node_directions[0].clone(),
            right: node_directions[1].clone()

        });
    }

    let mut curr_node: String = "AAA".to_string();
    while curr_node != "ZZZ".to_string() {
        for direction in directions.chars() {
            count += 1;
            match direction {
                'R' => {
                    curr_node = map_hash.get(&curr_node).expect("a valid node").right.to_string();
                },
                'L' => {
                    curr_node = map_hash.get(&curr_node).expect("a valid node").left.to_string();
                },
                _ => {
                    panic!("Invalid direction");
                }
            }
        }
    }
    return count
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_with_first_test() {
        let input_string = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)".to_string();
        assert_eq!(process(input_string), 2)
    }

    #[test]
    fn it_works_with_second_test() {
        let input_string = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)".to_string();
        assert_eq!(process(input_string), 6)
    }
}
