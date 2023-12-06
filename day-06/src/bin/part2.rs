use std::ops::Range;

fn main() {
    println!("Advent of Code: Day 6, Part 2");
    let input_string = "Time:        50     74     86     85
Distance:   242   1017   1691   1252".to_string();

    println!("{}", process(input_string));
}


fn get_num_game_wins(game_time: u64, record_distance: u64) -> u64 {
    let time_range = Range{
        start: 0,
        end: game_time
    };

    let min_time = time_range.clone().into_iter().find(|hold_seconds| {
        let time_remaining = game_time - hold_seconds;
        let speed = hold_seconds;
        speed * time_remaining > record_distance
    }).expect("a valid winner") as u64;
    let max_time = time_range.clone().into_iter().rev().find(|hold_seconds| {
        let time_remaining = game_time - hold_seconds;
        let speed = hold_seconds;
        speed * time_remaining > record_distance
    }).expect("a valid winner") as u64;
    max_time - min_time + 1
}

fn process(input_string: String) -> u64 {
    let mut parts = input_string.lines();
    let time: u64 = parts.next()
                         .unwrap_or_default()
                         .trim_start_matches("Time:")
                         .trim()
                         .replace(" ", "")
                         .parse::<u64>()
                         .expect("a valid int");
    let distance: u64 = parts.next()
                             .unwrap_or_default()
                             .trim_start_matches("Distance:")
                             .trim()
                             .replace(" ", "")
                             .parse::<u64>().expect("a valid int");

    get_num_game_wins(time, distance)

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_get_max_wins_for_first_example() {
        assert_eq!(get_num_game_wins(71530, 940200), 71503)
    }

    #[test]
    fn it_works() {
        let input_string = "Time:      7  15   30
Distance:  9  40  200".to_string();
        assert_eq!(process(input_string), 71503)
    }
}
