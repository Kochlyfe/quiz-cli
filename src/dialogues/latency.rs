use crate::Prompts;

pub fn latency_questions() -> Vec<Prompts> {
    return vec![
        Prompts {
            question: String::from("L1 cache reference"),
            answer: String::from("1ns"),
        },
        Prompts {
            question: String::from("Branch mispredict"),
            answer: String::from("3ns"),
        },
        Prompts {
            question: String::from("L2 cache reference"),
            answer: String::from("4ns"),
        },
        Prompts {
            question: String::from("Mutex lock/unlock"),
            answer: String::from("17ns"),
        },
        Prompts {
            question: String::from("Main memory reference"),
            answer: String::from("100ns"),
        },
        Prompts {
            question: String::from("Compress 1KB with Zippy"),
            answer: String::from("2000ns"),
        },
        Prompts {
            question: String::from("Send 2000 bytes over commodity network"),
            answer: String::from("50ns"),
        },
        Prompts {
            question: String::from("SSD random read"),
            answer: String::from("15ys"),
        },
        Prompts {
            question: String::from("Read 1_000_000 bytes sequentially from memory"),
            answer: String::from("3ys"),
        },
        Prompts {
            question: String::from("Round trip in same datacenter"),
            answer: String::from("500ys"),
        },
        Prompts {
            question: String::from("Read 1_000_000 bytes sequentially from SSD"),
            answer: String::from("50ys"),
        },
        Prompts {
            question: String::from("Disk seek"),
            answer: String::from("2ms"),
        },
        Prompts {
            question: String::from("Read 1_000_000 bytes sequentially from disk"),
            answer: String::from("825ys"),
        },
        Prompts {
            question: String::from("Packet roundtrip CA to Netherlands"),
            answer: String::from("150ms"),
        },
    ];
}
