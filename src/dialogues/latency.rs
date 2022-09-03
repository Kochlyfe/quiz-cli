use crate::Prompts;

pub fn latency_questions() -> Vec<Prompts> {
    return vec![
        Prompts {
            question: String::from("A mutex lock"),
            answer: String::from("5 ns"),
        },
        Prompts {
            question: String::from("Datacenter round-trip"),
            answer: String::from("100 ms"),
        },
    ];
}
