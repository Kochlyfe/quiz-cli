use clap::Parser;
use cstrainer::test::{Cli, Mode};

fn main() {
    let args = Cli::parse();

    match args.mode {
        Mode::Latency => println!("{:?}", Mode::generate(&Mode::Latency)),
        Mode::Po2 => println!("po2"),
    }
}
