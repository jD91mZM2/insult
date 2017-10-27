// All words are taken from
// http://ohsir-the-insult-simulator.wikia.com/wiki/All_of_the_words_in_Oh...
// Sir!!_The_Insult_Simulator
//
// What is referred to as "word" is actually not a word by the way.
// It's a part of a sentence.

extern crate rand;

mod words;
mod config;

use rand::{ThreadRng, Rng};
use std::process;
use words::*;

const AMOUNT: u8 = 9 + 2*2; // Board + Tea * 2
const MIN: u8 = 3;
const MAX: u8 = 5;

fn main() {
    let code = do_main();
    process::exit(code);
}
fn do_main() -> i32 {
    let wordstuple = match config::make_configs() {
        Ok(words) => words,
        Err(err) => {
            eprintln!("Error! Could not make configs: {}", err);
            return 1;
        },
    };
    let wordsfile = match WordsFile::parse_tuple(wordstuple) {
        Ok(words) => words,
        Err(err) => {
            eprintln!("{}", err);
            return 1;
        },
    };

    let mut rand = rand::thread_rng();
    let mut words = Vec::new();

    let mut num_nouns = 0;
    let mut num_verbs = 0;

    while words.len() < AMOUNT as usize {
        if num_nouns < MAX && rand.gen() {
            num_nouns += 1;
            words.push(wordsfile.gen_noun(&mut rand));
        } else if num_verbs < MAX && rand.gen() {
            num_verbs += 1;
            words.push(wordsfile.gen_verb(&mut rand));
        } else if num_nouns >= MIN && num_verbs >= MIN {
            match rand.gen::<u8>() % 10 {
                0 => words.push(wordsfile.gen_ending(&mut rand)),
                1 => words.push(Word::And),
                _ => {},
            }
        }
    }

    let mut gen = Generator {
        completed: Vec::new(),
        words: words,

        rand: rand
    };
    loop {
        gen.expect_noun();
        gen.expect_verb();

        if gen.rand.gen() && gen.has_and() && gen.has_noun() && gen.has_verb() {
            gen.expect_and();
            continue;
        }
        break;
    }
    if gen.rand.gen() && gen.has_ending() {
        gen.expect_ending();
    }

    println!("{}", gen.to_string());
    return 0;
}

struct Generator {
    completed: Vec<Word>,
    words: Vec<Word>,

    rand: ThreadRng
}
// Because specifying function return type is barely possible
// and `-> impl Iterator {` hasn't been stabilized yet
macro_rules! indexes {
    ($iter:expr) => {
        $iter.map(|item| item.0)
    }
}
impl Generator {
    fn sample(&mut self, words: &mut Vec<usize>) -> Word {
        assert!(!words.is_empty());
        self.words.remove(words[self.rand.gen::<usize>() % words.len()])
    }
    fn has_noun(&mut self) -> bool { self.words.iter().any(|item| item.is_noun()) }
    fn has_ending(&mut self) -> bool { self.words.iter().any(|item| item.is_ending()) }
    fn has_verb(&mut self) -> bool { self.words.iter().any(|item| item.is_verb()) }
    fn has_and(&mut self) -> bool { self.words.iter().any(|item| item.is_and()) }

    fn expect_noun(&mut self) {
        let mut nouns = indexes!(self.words.iter_mut().enumerate().filter(|&(_, ref word)| word.is_noun())).collect();
        let sample = self.sample(&mut nouns);
        self.completed.push(sample);

        if self.rand.gen() && self.has_and() && self.has_noun() {
            self.expect_and();
            self.expect_noun();
        }
    }
    fn expect_ending(&mut self) {
        let mut endings = indexes!(self.words.iter_mut().enumerate().filter(|&(_, ref word)| word.is_ending())).collect();
        let sample = self.sample(&mut endings);
        self.completed.push(sample);
    }
    fn expect_verb(&mut self) {
        let mut verbs = indexes!(self.words.iter_mut().enumerate().filter(|&(_, ref word)| word.is_verb())).collect();
        let verb = self.sample(&mut verbs);
        let noun = match verb {
            Word::Verb(Verb(noun, _)) => noun,
            _ => unreachable!()
        };
        self.completed.push(verb);
        if noun {
            if self.has_noun() {
                self.expect_noun();
            } else {
                self.completed.push(Word::Unfinished);
            }
        }
    }
    fn expect_and(&mut self) {
        let pos = self.words.iter().position(|word| word.is_and());
        self.completed.push(self.words.remove(pos.unwrap()));
    }

    fn to_string(&self) -> String {
        let mut he_she_it = false;
        let mut was_and = false;
        // 128 is just a guess.
        self.completed.iter().fold(String::with_capacity(128), |mut acc, word| {
            if word.is_ending() {
                acc.push(',');
            }
            if !acc.is_empty() && !word.is_unfinished() {
                acc.push(' ');
            }
            match *word {
                Word::Noun(new_he_she_it, ref noun) => {
                    he_she_it = new_he_she_it && !was_and;
                    acc.push_str(noun.as_str());
                },
                Word::Verb(ref verb) => {
                    acc.push_str(&verb.gen(he_she_it));
                },
                Word::Ending(ref ending) => acc.push_str(ending.as_str()),
                Word::And => acc.push_str("and"),
                Word::Unfinished => acc.push_str("... eh... uhnn...")
            }
            if was_and { was_and = false; }
            if word.is_and() { was_and = true; }
            acc
        })
    }
}
