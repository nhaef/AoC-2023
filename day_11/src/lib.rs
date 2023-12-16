use aoc_trait::AdventOfCodeSolution;
use space_image::SpaceImage;

mod space_image;

pub struct Solution(usize);

impl AdventOfCodeSolution for Solution {
    fn day() -> usize {
        11
    }

    fn name() -> &'static str {
        "--- Day 11: Cosmic Expansion ---"
    }

    fn solve_1(input: &str) -> Self {
        Self(SpaceImage::new(input, 1).get_sum_of_galaxy_distances())
    }

    fn solve_2(input: &str) -> Self {
        Self(SpaceImage::new(input, 999_999).get_sum_of_galaxy_distances())
    }
}

impl std::fmt::Display for Solution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "the shortest path between all pairs of galaxies is {}",
            self.0
        )
    }
}
