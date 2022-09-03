mod latency;

pub mod test {
    use crate::latency::latency_questions;
    use crate::Prompts;
    use clap::Parser;

    #[derive(clap::ValueEnum, Clone, Debug)]
    pub enum Mode {
        Latency,
        Po2,
    }

    impl Mode {
        pub fn generate(&self) -> Vec<Prompts> {
            match self {
                Mode::Latency => return latency_questions(),
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
}

#[derive(Debug)]
pub struct Prompts {
    pub question: String,
    pub answer: String,
}
