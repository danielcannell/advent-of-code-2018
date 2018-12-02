use docopt::Docopt;
use serde_derive::Deserialize;

mod day1;
mod day2;

const USAGE: &'static str = "
Advent of code 2018

Usage:
    advent-of-code-2018 <day>
    advent-of-code-2018 (-h | --help)

Options:
    -h --help   Show this help
";

#[derive(Debug, Deserialize)]
struct Args {
    arg_day: Option<u32>,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    match args.arg_day {
        Some(1) => day1::solve(),
        Some(2) => day2::solve(),
        _ => println!("Unknown day"),
    }
}
