use aoc_trait::AdventOfCodeSolution;
use light_beam::{get_max_number_of_energized_tiles, get_number_of_energized_tiles};

mod light_beam;

pub struct Solution(usize);

impl AdventOfCodeSolution for Solution {
    fn day() -> usize {
        16
    }

    fn name() -> &'static str {
        "--- Day 16: The Floor Will Be Lava ---"
    }

    fn solve_1(input: &str) -> Self {
        Self(get_number_of_energized_tiles(input))
    }

    fn solve_2(input: &str) -> Self {
        Self(get_max_number_of_energized_tiles(input))
    }
}

impl std::fmt::Display for Solution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "the amount of energized tiles is {}", self.0)
    }
}
