use aoc_trait::AdventOfCodeSolution;

mod mirror_1;
mod mirror_2;

pub struct Solution(usize);

impl AdventOfCodeSolution for Solution {
    fn name() -> &'static str {
        "--- Day 13: Point of Incidence ---"
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

    fn solve_1(input: &str) -> Self {
        Solution(mirror_1::summarize_all_notes(input))
    }

    fn solve_2(input: &str) -> Self {
        Solution(mirror_2::summarize_all_notes(input))
    }
}

impl std::fmt::Display for Solution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "the note summary number is {}", self.0)
    }
}
