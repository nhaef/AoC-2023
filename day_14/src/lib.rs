use aoc_trait::AdventOfCodeSolution;
use parabolic_reflector_1::get_total_load_on_north_support_beam;
use parabolic_reflector_2::get_total_load_on_north_support_beam_after_cycles;

mod parabolic_reflector_1;
mod parabolic_reflector_2;

pub struct Solution(usize);

impl AdventOfCodeSolution for Solution {
    fn day() -> usize {
        14
    }

    fn name() -> &'static str {
        "--- Day 14: Parabolic Reflector Dish ---"
    }

    fn solve_1(input: &str) -> Self {
        Self(get_total_load_on_north_support_beam(input))
    }

    fn solve_2(input: &str) -> Self {
        Self(get_total_load_on_north_support_beam_after_cycles(
            input,
            1_000_000_000,
        ))
    }
}

impl std::fmt::Display for Solution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "the total load on the north support beams is {}", self.0)
    }
}
