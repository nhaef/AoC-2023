use clap::Parser;
use cli::{Cli, CliInput, CliInputSub};
use colored::*;
use inputs::{download_puzzle_input, list_stored_puzzle_inputs, read_puzzle_input, read_puzzle_example_inputs};

mod cli;
mod inputs;

fn main() {
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
        || run_solution!(day_12::Solution),
        || run_solution!(day_13::Solution),
        || run_solution!(day_14::Solution),
        || run_solution!(day_15::Solution),
        || run_solution!(day_16::Solution),
    ];

    match Cli::parse().subcommand {
        cli::CliSub::Input(CliInput {
            subcommand: CliInputSub::Download(download),
        }) => {
            download_puzzle_input(download.day, &download.session_cookie);
        }
        cli::CliSub::Input(CliInput {
            subcommand: CliInputSub::List,
        }) => list_stored_puzzle_inputs(),
        cli::CliSub::Solve(solve) => {
            if cfg!(debug_assertions) {
                warn!("currently running in debug mode");
                warn!("consider using '--release' flag for optimal performance");
            }        
            match solve.day {
                Some(day) => solutions[day -1](),
                None => solutions.into_iter().for_each(|f| f()),
            }
        }
    };
}

type StdDuration = std::time::Duration;
struct Duration(pub StdDuration);
impl std::fmt::Display for Duration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut time_minutes = (self.0.as_secs() / 60).to_string().white();
        let mut time_seconds = (self.0.as_secs() % 60).to_string().white();
        let mut time_millis = self.0.subsec_millis().to_string().white();
        let mut time_micros = ((self.0.subsec_nanos() / 1_000) % 1_000)
            .to_string()
            .white();
        let mut time_nanos = (self.0.subsec_nanos() % 1_000).to_string().white();
        if self.0 < StdDuration::from_millis(1) {
            time_minutes = time_minutes.dimmed();
            time_seconds = time_seconds.dimmed();
            time_millis = time_millis.dimmed();
            time_micros = time_micros.green();
            time_nanos = time_nanos.green();
        } else if self.0 < StdDuration::from_millis(10) {
            time_minutes = time_minutes.dimmed();
            time_seconds = time_seconds.dimmed();
            time_millis = time_millis.green();
            time_micros = time_micros.green();
            time_nanos = time_nanos.green();
        } else if self.0 < StdDuration::from_millis(1000) {
            time_minutes = time_minutes.dimmed();
            time_seconds = time_seconds.dimmed();
            time_millis = time_millis.yellow();
            time_micros = time_micros.yellow();
            time_nanos = time_nanos.yellow();
        } else if self.0 < StdDuration::from_secs(60) {
            time_minutes = time_minutes.dimmed();
            time_seconds = time_seconds.red();
            time_millis = time_millis.red();
            time_micros = time_micros.red();
            time_nanos = time_nanos.red();
        } else {
            time_minutes = time_minutes.red();
            time_seconds = time_seconds.red();
            time_millis = time_millis.red();
            time_micros = time_micros.red();
            time_nanos = time_nanos.red();
        }
        format!(
            "{:0>2}:{:0>2}:{:0>4}.{:0>4}.{:0>4}",
            time_minutes, time_seconds, time_millis, time_micros, time_nanos
        )
        .fmt(f)
    }
}

#[macro_export]
macro_rules! run_solution {
    ($solution:path) => {{
        use aoc_trait::AdventOfCodeSolution;
        type Solution = $solution;
        println!("{}", Solution::name().white().bold());
        let day = Solution::day();

        // read puzzle input
        let input = read_puzzle_input(day);
        if input.is_none() {
            println!("{}", "The puzzle inputs could not be found!".red().bold());
            let command = format!("cargo run input download --session-cookie <your-session-cookie> --day {}", Solution::day()).white().bold();
            println!("{}{}{}", "Use '".red().bold(), command, "' to download the inputs".red().bold());
        }
        let input_examples_1 = read_puzzle_example_inputs(day, 1);
        let input_examples_2 = read_puzzle_example_inputs(day, 2);
        let max_name_width = input_examples_1.iter().chain(input_examples_2.iter()).fold(0, |acc, x| usize::max(acc, x.0.len() + 2));

        // run example inputs (part 1)
        for (name, input) in input_examples_1 {
            let (time, result) = measure_time!(Solution::solve_1(&input));
            println!("part 1  {:<width$}{}  {}", name, Duration(time), result, width = max_name_width);   
        }

        // run solution (part 1)
        if let Some(input) = &input {
            let (time, result) = measure_time!(Solution::solve_1(input));
            println!("part 1  {:<width$}{}  {}", "", Duration(time), result, width = max_name_width);    
        }

        // run example inputs (part 2)
        for (name, input) in input_examples_2 {
            let (time, result) = measure_time!(Solution::solve_2(&input));
            println!("part 2  {:<width$}{}  {}", name, Duration(time), result, width = max_name_width);   
        }

        // run solution (part 2)
        if let Some(input) = &input {
            let (time, result) = measure_time!(Solution::solve_2(input));
            println!("part 2  {:<width$}{}  {}", "", Duration(time), result, width = max_name_width);    
        }
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
    }};
}
