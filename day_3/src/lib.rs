use run_trait::AdventOfCodeDay;
use solution::Solution;

use crate::engine_schematic::EngineSchematic;

mod engine_schematic;
mod engine_schematic_symbol;
mod maybe_part;
mod solution;

pub struct Day3;

impl AdventOfCodeDay<Solution> for Day3 {
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
        let sum = EngineSchematic::from_raw_schematic(input).get_real_parts().iter().sum();
        Solution(solution::SolutionType::Part1, sum)
    }

    fn solve_2(input: &str) -> Solution {
        let sum = EngineSchematic::from_raw_schematic(input).get_gear_ratios().iter().sum();
        Solution(solution::SolutionType::Part2, sum)
    }
}
