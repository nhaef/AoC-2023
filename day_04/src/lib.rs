use card_list::{get_total_scratchcards_with_copies, get_total_winning_number_points};
use run_trait::AdventOfCodeSolution;

mod card;
mod card_list;

pub enum SolutionType {
    Part1,
    Part2,
}

pub struct Solution(pub SolutionType, pub u32);

impl AdventOfCodeSolution for Solution {
    fn name() -> &'static str {
        "--- Day 4: Scratchcards ---"
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
        Self(
            SolutionType::Part1,
            get_total_winning_number_points(input),
        )
    }

    fn solve_2(input: &str) -> Self {
        Self(
            SolutionType::Part2,
            get_total_scratchcards_with_copies(input),
        )
    }
}

impl std::fmt::Display for Solution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            SolutionType::Part1 => {
                write!(f, "The sum of all card scratchcard points is {}", self.1)
            }
            SolutionType::Part2 => {
                write!(f, "The total number of scratchcards is {}", self.1)
            }
        }
    }
}
