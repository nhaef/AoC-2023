pub trait AdventOfCodeDay<S>
where
    S: std::fmt::Display,
{
    fn name() -> &'static str;
    fn input_1() -> &'static str;
    fn input_2() -> &'static str;
    fn input_1_example() -> &'static str;
    fn input_2_example() -> &'static str;
    fn solve_1(input: &str) -> S;
    fn solve_2(input: &str) -> S;
}
