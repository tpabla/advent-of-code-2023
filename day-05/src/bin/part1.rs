fn main() {
    println!("Advent of Code, Day 5");
    let input_string = include_str!("../../examples/puzzle_input.txt");
    let min = process(input_string.to_string());
    println!("Solution: {min}")
}

#[derive(Debug, Clone)]
struct RangeMap {
    source_range_start: i64,
    destination_range_start: i64,
    range: i64,
}

#[derive(Debug, Clone)]
struct FromToMap {
    source_type: String,
    destination_type: String,
    ranges: Vec<RangeMap>,
}

impl FromToMap {
    fn parse_map(input_string: String) -> FromToMap {
        let mut parts = input_string.split('\n');
        let (source_type, destination_type) = {
            let parts: Vec<&str> = parts
                .next()
                .expect("a valid map name")
                .trim_end_matches("map:")
                .trim()
                .split('-')
                .filter(|&s| s != "to")
                .collect();
            if parts.len() == 2 {
                (parts[0], parts[1])
            } else {
                ("", "")
            }
        };

        let ranges: Vec<RangeMap> = parts
            .map(|s| {
                let values: Vec<_> = s
                    .trim()
                    .split_whitespace()
                    .map(|s| s.parse::<i64>().expect("a valid number"))
                    .collect();
                if values.len() == 3 {
                    RangeMap {
                        destination_range_start: values[0],
                        source_range_start: values[1],
                        range: values[2],
                    }
                } else {
                    panic!("Invalid Input")
                }
            })
            .collect();
        FromToMap {
            source_type: source_type.to_string(),
            destination_type: destination_type.to_string(),
            ranges: ranges,
        }
    }

    fn get_destination(&self, seed: i64) -> i64 {
        match self.ranges.iter().find(|range| {
            range.source_range_start <= seed && seed <= range.source_range_start + range.range
        }) {
            None => seed,
            Some(range) => seed - range.source_range_start + range.destination_range_start,
        }
    }
}

fn process(input_string: String) -> i64 {
    let mut parts = input_string.split("\n\n");
    let seeds: Vec<i64> = parts
        .next()
        .expect("Valid seed list")
        .trim_start_matches("seeds:")
        .split_whitespace()
        .map(|s| s.parse::<i64>().expect("a valid number"))
        .collect();

    let maps: Vec<_> = parts.map(|s| s.trim()).collect();
    let maps: Vec<_> = maps
        .iter()
        .map(|map| FromToMap::parse_map(map.to_string()))
        .collect();

    let mut locations: Vec<i64> = vec![];
    for seed in &seeds {
        let mut location = *seed;
        for map in &maps {
            location = map.get_destination(location.clone());
        }
        locations.push(location);
    }
    dbg!(locations.clone());
    *locations.iter().min().expect("a min number is present")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let input = include_str!("../../examples/example_input.txt");
        assert_eq!(process(input.to_string()), 35);
    }
}
