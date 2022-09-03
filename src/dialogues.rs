use dialoguer::{theme::ColorfulTheme, Input};

use crate::Prompts;

pub mod latency;

pub fn generate_dialogues(prompts: Vec<Prompts>) -> Vec<Prompts> {
    let mut wrong_answers: Vec<Prompts> = vec![];

    for prompt in prompts {
        let answer: String = Input::with_theme(&ColorfulTheme::default())
            .with_prompt(&prompt.question)
            .interact_text()
            .unwrap();

        if answer != prompt.answer {
            wrong_answers.push(Prompts {
                question: String::from(&prompt.question),
                answer: String::from(&prompt.answer),
            });
        }
    }

    wrong_answers
}
