use crate::markov::MarkovBot;

pub mod markov;

fn main() {
    let mut bot = MarkovBot::new();
    bot.add_link("", "test1");
    bot.add_link("", "test2");

    let mut a = 0;
    let mut b = 0;
    for _ in 0..10000 {
        match bot.chain("") {
            "test1" => a += 1,
            "test2" => b += 1,
            _ => panic!("unknown answer"),
        }
    }

    println!("{} {}", a, b);
}
