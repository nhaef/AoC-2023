pub trait AdventOfCodeSolution
where
    Self: std::fmt::Display,
{
    fn day() -> usize;
    fn name() -> &'static str;
    fn solve_1(input: &str) -> Self;
    fn solve_2(input: &str) -> Self;
}
