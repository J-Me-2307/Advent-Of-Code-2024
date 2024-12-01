use clap::Parser;

mod args;
mod puzzles;

fn main() {
    let args = args::Args::parse();

    match args.day {
        1 => puzzles::day_one::run_day_one(),
        _ => println!("This day hasn't been solved yet!"),
    }
}
