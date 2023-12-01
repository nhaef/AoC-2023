use std::time::Instant;

use day_1::Day1;
use run_trait::*;

fn main() {
    run_day!(Day1, "--- Day 1: Trebuchet?! ---");
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
    ($s:expr) => {
        {
            let now = Instant::now();
            let result = $s;
            (now.elapsed().as_millis(), result)
        }
    };
}