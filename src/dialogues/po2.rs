use crate::Prompts;

pub fn po2_questions() -> Vec<Prompts> {
    let powers = vec![10, 20, 30, 40, 50];
    return powers
        .iter()
        .map(|p| Prompts {
            question: String::from(format!("How many bytes are 2 to the power of {}", p)),
            answer: String::from(format!("{}", i64::pow(2, *p))),
        })
        .collect();
}
