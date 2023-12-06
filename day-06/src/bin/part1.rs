use std::ops::Range;

fn main() {
    println!("Advent of Code: Day 6, Part 1");
    let input_string = "Time:        50     74     86     85
Distance:   242   1017   1691   1252".to_string();

    println!("{}", process(input_string));
}


fn get_num_game_wins(game_time: u32, record_distance: u32) -> u32 {
    let time_range = Range{
        start: 0,
        end: game_time
    };

    time_range.into_iter().map(|hold_seconds| {
        let time_remaining = game_time - hold_seconds;
        let speed = hold_seconds;
        speed * time_remaining
    }).filter(|distance| distance > &record_distance).count() as u32
}

fn process(input_string: String) -> u32{
    let mut parts = input_string.lines();
    let times: Vec<u32> = parts.next().unwrap_or_default().trim_start_matches("Time:").trim().split_whitespace().map(|time| time.parse::<u32>().expect("a valid int")).collect();
    let distance: Vec<u32> = parts.next().unwrap_or_default().trim_start_matches("Distance:").trim().split_whitespace().map(|time| time.parse::<u32>().expect("a valid int")).collect();

    let time_distance: Vec<(u32, u32)> = times.into_iter().zip(distance.into_iter()).collect();
    time_distance.into_iter().map(|arg| get_num_game_wins(arg.0, arg.1)).product()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_get_max_wins_for_first_example() {
        assert_eq!(get_num_game_wins(7, 9), 4)
    }

    #[test]
    fn it_works() {
        let input_string = "Time:      7  15   30
Distance:  9  40  200".to_string();
        assert_eq!(process(input_string), 288)
    }
}
