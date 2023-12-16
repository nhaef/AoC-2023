use aoc_trait::AdventOfCodeSolution;
use hash::get_hash_sum_of_init_sequence_steps;
use hashmap::get_focussing_power;

mod hash;
mod hashmap;

pub enum SolutionType {
    Part1,
    Part2,
}

pub struct Solution(SolutionType, usize);

impl AdventOfCodeSolution for Solution {
    fn day() -> usize {
        15
    }

    fn name() -> &'static str {
        "--- Day 15: Lens Library ---"
    }

    fn solve_1(input: &str) -> Self {
        Self(SolutionType::Part1, get_hash_sum_of_init_sequence_steps(input))
    }

    fn solve_2(input: &str) -> Self {
        Self(SolutionType::Part2, get_focussing_power(input))
    }
}

impl std::fmt::Display for Solution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            SolutionType::Part1 => write!(f, "the sum of hashed initialization sequence steps is {}", self.1),
            SolutionType::Part2 => write!(f, "the focusing power of the lens configuration is {}", self.1),
        }
    }
}
