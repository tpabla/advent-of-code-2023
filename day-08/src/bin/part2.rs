use std::{collections::HashMap};

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

fn process(input_string: String) -> u64 {
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

    curr_nodes.sort();
    dbg!(curr_nodes.clone());

    let mut path_count: Vec<u64> = curr_nodes.clone().iter().map( |curr_node| {
        let mut count =00;
        let mut node = curr_node.clone();
        while !node.ends_with('Z') {
            for direction in directions.chars() {
                count += 1;
                match direction {
                    'R' => {
                        node = map_hash.get(&node).expect("a valid node").right.clone();

                    },
                    'L' => {
                        node = map_hash.get(&node).expect("a valid node").left.clone();
                    },
                    _ => {
                        panic!("Invalid direction");
                    }
                }
            }
        }
        count
    }).collect();
    path_count.sort();
    dbg!(path_count.clone());

    let output: u64 = path_count.into_iter().fold(1, num_integer::lcm);
    output

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
