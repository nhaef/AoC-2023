pub enum SolutionType {
    Part1,
    Part2,
}

pub struct Solution(pub SolutionType, pub u32);

impl std::fmt::Display for Solution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            SolutionType::Part1 => write!(f, "The sum of possible game IDs is {}", self.1),
            SolutionType::Part2 => write!(
                f,
                "The sum of the power of smallest possible sets is {}",
                self.1
            ),
        }
    }
}
