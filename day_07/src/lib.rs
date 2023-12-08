use run_trait::AdventOfCodeDay;
use solution::Solution;
use sort::{get_total_winnings_1, get_total_winnings_2, input_to_hands_2};

use crate::sort::input_to_hands_1;

mod hand;
mod solution;
mod sort;

pub struct Day7;

impl AdventOfCodeDay<Solution> for Day7 {
    fn name() -> &'static str {
        "--- Day 7: Camel Cards ---"
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

    fn solve_1(input: &str) -> Solution {
        Solution(get_total_winnings_1(input_to_hands_1(input)))
    }

    fn solve_2(input: &str) -> Solution {
        Solution(get_total_winnings_2(input_to_hands_2(input)))
    }
}

#[allow(dead_code)]
pub fn print_hands(hands: &Vec<hand::Hand>) {
    println!("--- Hands --- ");
    for hand in hands {
        println!("{}", hand)
    }
}