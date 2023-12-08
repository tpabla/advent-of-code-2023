use std::{collections::HashMap, io::Write};

fn main() {
    let input_string = include_str!("../../examples/puzzle_input.txt").to_string();
    println!("Advent of Code: Day 7, Part 2");
    println!("Solution: {}", process(input_string));
}


#[derive(Debug)]
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

    let mut curr_nodes: Vec<_> = map_hash
        .values()
        .map(|node| node)
        .filter(|node| node.current.ends_with("A"))
        .map(|s| s.current.to_string()).collect();

    let mut count: u32 = 0;
    dbg!(curr_nodes.clone());
    loop {
        for direction in directions.chars() {
            let mut updated_nodes: Vec<String> = Default::default();
            let mut finished_nodes: Vec<bool> = Default::default();
            for curr_node in &curr_nodes{
                match direction {
                    'R' => {
                        updated_nodes.push(map_hash.get(curr_node).expect("a valid node").right.clone());

                    },
                    'L' => {
                        updated_nodes.push(map_hash.get(curr_node).expect("a valid node").left.clone())
                    },
                    _ => {
                        panic!("Invalid direction");
                    }
                }
                if curr_node.ends_with('Z') {
                    finished_nodes.push(true);
                }
                // println!("{}", curr_node.clone());
            }
            curr_nodes = updated_nodes.clone();
            if finished_nodes.len() == curr_nodes.len() {
                return count
            }
            count += 1;
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_with_first_test() {
        let input_string = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)".to_string();
        assert_eq!(process(input_string), 6)
    }
}
