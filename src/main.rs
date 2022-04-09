pub mod markov;

fn main() {
    let a = "testing this thing";
    let a = a.split(' ');
    for s in a {
        println!("{}", s);
    }
}
