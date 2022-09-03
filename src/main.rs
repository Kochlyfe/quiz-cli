use clap::Parser;
use cstrainer::dialogues::generate_dialogues;
use cstrainer::test::{format_answers, Cli, Mode};

fn main() {
    let args = Cli::parse();

    let questions = match args.mode {
        Mode::Latency => Mode::Latency.generate(),
        Mode::Po2 => Mode::Po2.generate(),
    };

    let wrong_answers = generate_dialogues(questions);
    format_answers(wrong_answers);
}
