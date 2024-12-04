use clap::Parser;

mod args;
mod puzzles;
mod file_helper;

fn main() {
    let args = args::Args::parse();

    match args.day {
        1 => puzzles::day1::run(),
        2 => puzzles::day2::run(),
        3 => puzzles::day3::run(),
        4 => puzzles::day4::run(),
        _ => println!("This day hasn't been solved yet!"),
    }
}
