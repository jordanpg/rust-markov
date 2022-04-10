use std::io::{stdout, Write};

use text_io::read;

use crate::markov::MarkovBot;

pub mod markov;

fn main() {
    let mut bot = MarkovBot::new();
    println!("Start typing to talk to the chatbo!");
    loop {
        print!("You: ");
        stdout().flush().unwrap();
        let input: String = read!("{}\n");
        if input.trim().len() < 1 {
            break;
        }
        bot.ingest(&input);
        println!("Monty: {}", bot.generate(25));
    }
}
