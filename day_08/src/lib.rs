#![feature(ascii_char)]
use run_trait::AdventOfCodeDay;
use solution::Solution;
use state_machine::{find_a_to_z_steps, find_aaa_to_zzz_steps};

mod node;
mod solution;
mod state_machine;

pub struct Day8;

impl AdventOfCodeDay<Solution> for Day8 {
    fn name() -> &'static str {
        "--- Day 8: Haunted Wasteland ---"
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

    fn solve_1(input: &str) -> Solution {
        Solution(find_aaa_to_zzz_steps(input))
    }

    fn solve_2(input: &str) -> Solution {
        Solution(find_a_to_z_steps(input))
    }
}
