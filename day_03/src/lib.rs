use run_trait::AdventOfCodeSolution;

use crate::engine_schematic::EngineSchematic;

mod engine_schematic;
mod engine_schematic_symbol;
mod maybe_part;

pub enum SolutionType {
    Part1,
    Part2,
}

pub struct Solution(pub SolutionType, pub u32);

impl AdventOfCodeSolution for Solution {
    fn name() -> &'static str {
        "--- Day 3: Gear Ratios ---"
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
        let sum = EngineSchematic::from_raw_schematic(input)
            .get_real_parts()
            .iter()
            .sum();
        Self(SolutionType::Part1, sum)
    }

    fn solve_2(input: &str) -> Self {
        let sum = EngineSchematic::from_raw_schematic(input)
            .get_gear_ratios()
            .iter()
            .sum();
        Self(SolutionType::Part2, sum)
    }
}

impl std::fmt::Display for Solution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            SolutionType::Part1 => write!(f, "The sum of all part numbers is {}", self.1),
            SolutionType::Part2 => write!(f, "The sum of all gear ratios is {}", self.1),
        }
    }
}
