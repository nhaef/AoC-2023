use std::time::Instant;

use day_1::Day1;
use day_2::Day2;
use day_3::Day3;
use day_4::Day4;
use run_trait::*;

fn main() {
    run_day!(Day1, "--- Day 1: Trebuchet?! ---");
    run_day!(Day2, "--- Day 2: Cube Conundrum ---");
    run_day!(Day3, "--- Day 3: Gear Ratios ---");
    run_day!(Day4, "--- Day 4: Scratchcards ---");
}

#[macro_export]
macro_rules! run_day {
    ($day:ident, $name:literal) => {
        println!($name);
        let (time, result) = measure_time!($day::solve_1($day::input_1_example()));
        println!("part 1 (example) ({:08} ms) >> {}", time, result);
        let (time, result) = measure_time!($day::solve_1($day::input_1()));
        println!("part 1           ({:08} ms) >> {}", time, result);
        let (time, result) = measure_time!($day::solve_2($day::input_2_example()));
        println!("part 2 (example) ({:08} ms) >> {}", time, result);
        let (time, result) = measure_time!($day::solve_2($day::input_2()));
        println!("part 2           ({:08} ms) >> {}", time, result);
    };
}

#[macro_export]
macro_rules! measure_time {
    ($s:expr) => {{
        let now = Instant::now();
        let result = $s;
        (now.elapsed().as_millis(), result)
    }};
}
