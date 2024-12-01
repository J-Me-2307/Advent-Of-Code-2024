use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    /// The day that should be executed
    #[arg(short, long)]
    pub day: u8
}
