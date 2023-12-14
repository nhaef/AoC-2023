use aoc_trait::AdventOfCodeSolution;
use game_list::{solve_game_list_1, solve_game_list_2};

mod cube_set;
mod game;
mod game_list;

pub enum SolutionType {
    Part1,
    Part2,
}

pub struct Solution(SolutionType, pub u32);

impl AdventOfCodeSolution for Solution {
    fn name() -> &'static str {
        "--- Day 2: Cube Conundrum ---"
    }

    fn input_1() -> &'static str {
        include_str!("../inputs/input_puzzle_1")
    }

    fn input_2() -> &'static str {
        include_str!("../inputs/input_puzzle_2")
    }

    fn input_1_example() -> &'static str {
        include_str!("../inputs/input_example")
    }

    fn input_2_example() -> &'static str {
        include_str!("../inputs/input_example")
    }

    fn solve_1(input: &str) -> Self {
        Self(SolutionType::Part1, solve_game_list_1(input))
    }

    fn solve_2(input: &str) -> Self {
        Self(SolutionType::Part2, solve_game_list_2(input))
    }
}

impl std::fmt::Display for Solution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            SolutionType::Part1 => write!(f, "The sum of possible game IDs is {}", self.1),
            SolutionType::Part2 => write!(
                f,
                "The sum of the power of smallest possible sets is {}",
                self.1
            ),
        }
    }
}
