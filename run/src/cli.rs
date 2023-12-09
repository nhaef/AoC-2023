use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, about, long_about = None)]
pub struct Args {
    /// Day to execute
    #[arg(short, long)]
    pub day: Option<usize>,
}