use clap::Parser;
use colored::*;

use crate::cli::Args;

mod cli;

fn main() {

    if cfg!(debug_assertions) {
        warn!("currently running in debug mode");
        warn!("consider using '--release' flag for optimal performance");
    }

    let solutions = vec![
        || run_solution!(day_01::Solution),
        || run_solution!(day_02::Solution),
        || run_solution!(day_03::Solution),
        || run_solution!(day_04::Solution),
        || run_solution!(day_05::Solution),
        || run_solution!(day_06::Solution),
        || run_solution!(day_07::Solution),
        || run_solution!(day_08::Solution),
        || run_solution!(day_09::Solution),
        || run_solution!(day_10::Solution),
        || run_solution!(day_11::Solution),
        || run_solution!(day_13::Solution),
    ];

    let args = Args::parse();
    match args.day {
        Some(day) => solutions[day - 1](),
        None => solutions.into_iter().for_each(|f| f()),
    }
}

type StdDuration = std::time::Duration;
struct Duration(pub StdDuration);
impl std::fmt::Display for Duration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut time_minutes = (self.0.as_secs() / 60).to_string().white();
        let mut time_seconds = (self.0.as_secs() % 60).to_string().white();
        let mut time_millis = self.0.subsec_millis().to_string().white();
        let mut time_micros = ((self.0.subsec_nanos() / 1_000) % 1_000).to_string().white();
        let mut time_nanos = (self.0.subsec_nanos() % 1_000).to_string().white();
        if self.0 < StdDuration::from_millis(1) {
            time_minutes = time_minutes.dimmed();
            time_seconds = time_seconds.dimmed();
            time_millis = time_millis.dimmed();
            time_micros = time_micros.green();
            time_nanos = time_nanos.green();
        }
        else if self.0 < StdDuration::from_millis(10) {
            time_minutes = time_minutes.dimmed();
            time_seconds = time_seconds.dimmed();
            time_millis = time_millis.green();
            time_micros = time_micros.green();
            time_nanos = time_nanos.green();
        }
        else if self.0 < StdDuration::from_millis(1000) {
            time_minutes = time_minutes.dimmed();
            time_seconds = time_seconds.dimmed();
            time_millis = time_millis.yellow();
            time_micros = time_micros.yellow();
            time_nanos = time_nanos.yellow();
        }
        else if self.0 < StdDuration::from_secs(60) {
            time_minutes = time_minutes.dimmed();
            time_seconds = time_seconds.red();
            time_millis = time_millis.red();
            time_micros = time_micros.red();
            time_nanos = time_nanos.red();
        }
        else {
            time_minutes = time_minutes.red();
            time_seconds = time_seconds.red();
            time_millis = time_millis.red();
            time_micros = time_micros.red();
            time_nanos = time_nanos.red();
        }
        format!("{:0>2}:{:0>2}:{:0>4}.{:0>4}.{:0>4}", time_minutes, time_seconds, time_millis, time_micros, time_nanos).fmt(f)
    }
}

#[macro_export]
macro_rules! run_solution {
    ($solution:path) => {{
        use aoc_trait::AdventOfCodeSolution;
        type Solution = $solution;
        println!("{}", Solution::name().white().bold());
        let (time, result) = measure_time!(Solution::solve_1(Solution::input_1_example()));
        println!("part 1 (example)  {} >> {}", Duration(time), result);
        let (time, result) = measure_time!(Solution::solve_1(Solution::input_1()));
        println!("part 1            {} >> {}", Duration(time), result);
        let (time, result) = measure_time!(Solution::solve_2(Solution::input_2_example()));
        println!("part 2 (example)  {} >> {}", Duration(time), result);
        let (time, result) = measure_time!(Solution::solve_2(Solution::input_2()));
        println!("part 2            {} >> {}", Duration(time), result);
    }};
}


#[macro_export]
macro_rules! measure_time {
    ($e:expr) => {{
        let now = std::time::Instant::now();
        let result = $e;
        (now.elapsed(), result)
    }};
}

#[macro_export]
macro_rules! warn {
    ($s:literal) => {{
        println!("warn: {}", $s.yellow().bold())
    }}
}