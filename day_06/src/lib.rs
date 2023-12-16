use aoc_trait::AdventOfCodeSolution;
use boat_race::{get_boat_race, get_boat_races, get_number_of_ways_to_win_race};

mod boat_race;

pub struct Solution(u64);

impl AdventOfCodeSolution for Solution {
    fn day() -> usize {
        6
    }

    fn name() -> &'static str {
        "--- Day 6: Wait For It ---"
    }

    fn solve_1(input: &str) -> Self {
        let ways_to_win_product = get_boat_races(input)
            .into_iter()
            .map(|r| get_number_of_ways_to_win_race(r))
            .fold(1, |acc, v| acc * v);
        Self(ways_to_win_product)
    }

    fn solve_2(input: &str) -> Self {
        Self(get_number_of_ways_to_win_race(get_boat_race(input)))
    }
}

impl std::fmt::Display for Solution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "The product of number of ways to beat the record is {}",
            self.0
        )
    }
}
