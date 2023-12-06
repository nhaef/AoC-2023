pub struct Solution(pub i64);

impl std::fmt::Display for Solution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "The lowest location number is {}", self.0)
    }
}
