use clap::{Parser, Subcommand};

#[derive(Parser)]
/// Welcome to my AoC solution runner!
pub struct Cli {
    #[command(subcommand)]
    pub subcommand: CliSub,
}

#[derive(Subcommand)]
pub enum CliSub {
    Solve(CliSolve),
    Input(CliInput),
}

/// Executes the specified AoC solution(s).
#[derive(Parser)]
pub struct CliSolve {
    /// Execute specific day
    #[arg(short, long)]
    pub day: Option<usize>,

    /// Execute only inputs with note
    #[arg(short, long)]
    pub example: bool,
}

/// Groups subcommands related to puzzle inputs.
#[derive(Parser)]
pub struct CliInput {
    #[command(subcommand)]
    pub subcommand: CliInputSub,
}

#[derive(Subcommand)]
pub enum CliInputSub {
    /// Lists locally stored puzzle inputs.
    List,
    Download(CliInputDownload),
}

/// Fetches the puzzle inputs from the AoC server.
#[derive(Parser)]
pub struct CliInputDownload {
    /// The day for which the puzzle input should be fetched.
    #[arg(short, long)]
    pub day: usize,

    /// The AoC session cookie which is used to authenticate the request.
    #[arg(short, long)]
    pub session_cookie: String,
}
