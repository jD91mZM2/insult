extern crate insult;
extern crate rand;

fn main() {
    let wordsfile = insult::open_default();
    let sentence = wordsfile.generate(rand::thread_rng());
    println!("{}", sentence);
}
