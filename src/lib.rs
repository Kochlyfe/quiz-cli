pub mod dialogues;

#[derive(Debug)]
pub struct Prompts {
    pub question: String,
    pub answer: String,
}

pub mod test {
    use crate::dialogues::latency;
    use crate::Prompts;
    use clap::Parser;

    #[derive(clap::ValueEnum, Clone, Debug)]
    pub enum Mode {
        Latency,
        Po2,
    }

    impl Mode {
        pub fn generate(self: Self) -> Vec<Prompts> {
            match self {
                Mode::Latency => return latency::latency_questions(),
                Mode::Po2 => return vec![],
            }
        }
    }

    #[derive(Parser, Debug)]
    pub struct Cli {
        // Can add default_value_t = Mode::Latency if I want that to be default behaviour
        #[clap(short, long, value_enum)]
        pub mode: Mode,
    }

    pub fn format_answers(wrong_answers: Vec<Prompts>) {
        match wrong_answers.len() {
            0 => println!("Well done, you answered everything correct"),
            _ => {
                println!(
                    "You had {} wrong answers. The correct answers are:",
                    wrong_answers.len()
                );
                wrong_answers
                    .into_iter()
                    .for_each(|p| println!("- {}: {}", p.question, p.answer));
            }
        }
    }
}
