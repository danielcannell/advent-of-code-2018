use docopt::Docopt;
use serde_derive::Deserialize;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;

const USAGE: &str = "
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
        Some(3) => day3::solve(),
        Some(4) => day4::solve(),
        Some(5) => day5::solve(),
        Some(6) => day6::solve(),
        Some(7) => day7::solve(),
        Some(8) => day8::solve(),
        Some(9) => day9::solve(),
        _ => println!("Unknown day"),
    }
}
