pub struct Solution(pub u32);

impl std::fmt::Display for Solution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "The sum of possible game IDs is {}", self.0)
    }
}
