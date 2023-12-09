pub trait AdventOfCodeSolution
where
    Self: std::fmt::Display,
{
    fn name() -> &'static str;
    fn input_1() -> &'static str;
    fn input_2() -> &'static str;
    fn input_1_example() -> &'static str;
    fn input_2_example() -> &'static str;
    fn solve_1(input: &str) -> Self;
    fn solve_2(input: &str) -> Self;
}
