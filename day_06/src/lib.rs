use boat_race::{get_boat_race, get_boat_races, get_number_of_ways_to_win_race};
use run_trait::AdventOfCodeDay;
use solution::Solution;

mod boat_race;
mod solution;

pub struct Day6;

impl AdventOfCodeDay<Solution> for Day6 {
    fn name() -> &'static str {
        "--- Day 6: Wait For It ---"
    }

    fn input_1() -> &'static str {
        include_str!("../inputs/input_puzzle")
    }

    fn input_2() -> &'static str {
        include_str!("../inputs/input_puzzle")
    }

    fn input_1_example() -> &'static str {
        include_str!("../inputs/input_example")
    }

    fn input_2_example() -> &'static str {
        include_str!("../inputs/input_example")
    }

    fn solve_1(input: &str) -> Solution {
        let ways_to_win_product = get_boat_races(input)
            .into_iter()
            .map(|r| get_number_of_ways_to_win_race(r))
            .fold(1, |acc, v| acc * v);
        Solution(ways_to_win_product)
    }

    fn solve_2(input: &str) -> Solution {
        Solution(get_number_of_ways_to_win_race(get_boat_race(input)))
    }
}
