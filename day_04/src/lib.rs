use aoc_trait::AdventOfCodeSolution;
use card_list::{get_total_scratchcards_with_copies, get_total_winning_number_points};

mod card;
mod card_list;

pub enum SolutionType {
    Part1,
    Part2,
}

pub struct Solution(SolutionType, pub u32);

impl AdventOfCodeSolution for Solution {
    fn day() -> usize {
        4
    }

    fn name() -> &'static str {
        "--- Day 4: Scratchcards ---"
    }

    fn solve_1(input: &str) -> Self {
        Self(SolutionType::Part1, get_total_winning_number_points(input))
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
                write!(f, "the sum of all card scratchcard points is {}", self.1)
            }
            SolutionType::Part2 => {
                write!(f, "the total number of scratchcards is {}", self.1)
            }
        }
    }
}
