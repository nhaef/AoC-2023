use card_list::{get_total_scratchcards_with_copies, get_total_winning_number_points};
use run_trait::AdventOfCodeDay;
use solution::Solution;

mod card;
mod card_list;
mod solution;

pub struct Day4;

impl AdventOfCodeDay<Solution> for Day4 {
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

    fn solve_1(input: &str) -> Solution {
        Solution(
            solution::SolutionType::Part1,
            get_total_winning_number_points(input),
        )
    }

    fn solve_2(input: &str) -> Solution {
        Solution(
            solution::SolutionType::Part2,
            get_total_scratchcards_with_copies(input),
        )
    }
}
