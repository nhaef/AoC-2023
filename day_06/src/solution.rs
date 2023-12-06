pub struct Solution(pub u64);

impl std::fmt::Display for Solution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "The product of number of ways to beat the record is {}",
            self.0
        )
    }
}
