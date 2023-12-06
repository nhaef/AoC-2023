use std::time::Instant;

use day_01::Day1;
use day_02::Day2;
use day_03::Day3;
use day_04::Day4;
use day_05::Day5;
use day_06::Day6;
use run_trait::*;

fn main() {
    run_day!(Day1);
    run_day!(Day2);
    run_day!(Day3);
    run_day!(Day4);
    run_day!(Day5);
    run_day!(Day6);
}

#[macro_export]
macro_rules! run_day {
    ($day:ident) => {
        println!("{}", $day::name());
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
