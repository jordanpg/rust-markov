use crate::markov::MarkovBot;

pub mod markov;

fn main() {
    let mut bot = MarkovBot::new();
    bot.ingest("here is a test sentence that should be reproduced");
    println!("{}", bot.generate(25));
}
