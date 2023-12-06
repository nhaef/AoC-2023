use game_list::{solve_game_list_1, solve_game_list_2};
use run_trait::AdventOfCodeDay;
use solution::Solution;

mod cube_set;
mod game;
mod game_list;
mod solution;

pub struct Day2;

impl AdventOfCodeDay<Solution> for Day2 {
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

    fn solve_1(input: &str) -> Solution {
        Solution(solution::SolutionType::Part1, solve_game_list_1(input))
    }

    fn solve_2(input: &str) -> Solution {
        Solution(solution::SolutionType::Part2, solve_game_list_2(input))
    }
}
