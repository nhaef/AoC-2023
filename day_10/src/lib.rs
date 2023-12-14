use aoc_trait::AdventOfCodeSolution;
use walker_1::get_steps_to_farthest_point;
use walker_2::get_enclosed_tiles;

mod maze;
mod walker_1;
mod walker_2;

pub enum SolutionType {
    Part1,
    Part2,
}

pub struct Solution(SolutionType, u32);

impl AdventOfCodeSolution for Solution {
    fn name() -> &'static str {
        "--- Day 10: Pipe Maze ---"
    }

    fn input_1() -> &'static str {
        include_str!("../inputs/input_puzzle")
    }

    fn input_2() -> &'static str {
        include_str!("../inputs/input_puzzle")
    }

    fn input_1_example() -> &'static str {
        include_str!("../inputs/input_example_1")
    }

    fn input_2_example() -> &'static str {
        include_str!("../inputs/input_example_2")
    }

    fn solve_1(input: &str) -> Self {
        Self(SolutionType::Part1, get_steps_to_farthest_point(input))
    }

    fn solve_2(input: &str) -> Self {
        Self(SolutionType::Part2, get_enclosed_tiles(input))
    }
}

impl std::fmt::Display for Solution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            SolutionType::Part1 => write!(
                f,
                "the amount of steps to the farthest point in the loop is {}",
                self.1
            ),
            SolutionType::Part2 => {
                write!(f, "the amount of tiles enclosed by the loop is {}", self.1)
            }
        }
    }
}
