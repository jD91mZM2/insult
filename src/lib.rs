// All words are taken from
// http://ohsir-the-insult-simulator.wikia.com/wiki/All_of_the_words_in_Oh...
// Sir!!_The_Insult_Simulator
//
// What is referred to as "word" is actually not a word by the way.
// It's a part of a sentence.

extern crate rand;

use rand::{ThreadRng, Rng};
use std::fmt::{self, Display};

mod words;

pub use words::*;

const AMOUNT: u8 = 9 + 2*2; // Board + Tea * 2
const MIN: u8 = 3;
const MAX: u8 = 5;

pub struct WordsFile {
    pub nouns:   Vec<(bool, String)>,
    pub endings: Vec<(bool, String)>,
    pub verbs:   Vec<(bool, String)>
}
impl WordsFile {
    pub fn gen_noun(&self, rand: &mut ThreadRng) -> Word {
        let &(he_she_it, ref word) = &self.nouns[rand.gen::<usize>() % self.nouns.len()];
        Word::Noun(he_she_it, word.clone())
    }
    pub fn gen_ending(&self, rand: &mut ThreadRng) -> Word {
        let &(_, ref word) = &self.endings[rand.gen::<usize>() % self.endings.len()];
        Word::Ending(word.clone())
    }
    pub fn gen_verb(&self, rand: &mut ThreadRng) -> Word {
        let &(noun, ref word) = &self.verbs[rand.gen::<usize>() % self.verbs.len()];
        Word::Verb(Verb(noun, word.clone()))
    }
    pub fn generate(&self) -> Generator {
        let mut rand = rand::thread_rng();
        let mut words = Vec::new();

        let mut num_nouns = 0;
        let mut num_verbs = 0;

        while words.len() < AMOUNT as usize {
            if num_nouns < MAX && rand.gen() {
                num_nouns += 1;
                words.push(self.gen_noun(&mut rand));
            } else if num_verbs < MAX && rand.gen() {
                num_verbs += 1;
                words.push(self.gen_verb(&mut rand));
            } else if num_nouns >= MIN && num_verbs >= MIN {
                match rand.gen::<u8>() % 10 {
                    0 => words.push(self.gen_ending(&mut rand)),
                    1 => words.push(Word::And),
                    _ => {},
                }
            }
        }

        let mut gen = Generator::new(words);

        gen.expect_noun(None);
        loop {
            gen.expect_verb();

            if gen.rand.gen() && gen.has_and() && gen.has_verb() {
                gen.expect_and();
                if gen.rand.gen() && gen.has_noun() {
                    gen.expect_noun(None);
                }
                continue;
            }
            break;
        }
        if gen.rand.gen() && gen.has_ending() {
            gen.expect_ending();
        }

        gen
    }
}

pub struct Generator {
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
    pub fn new(words: Vec<Word>) -> Self {
        Self {
            completed: Vec::new(),
            words: words,

            rand: rand::thread_rng()
        }
    }
    pub fn sample(&mut self, words: &mut Vec<usize>) -> Word {
        assert!(!words.is_empty());
        self.words.remove(*self.rand.choose(&words).unwrap())
    }
    pub fn has_noun(&mut self) -> bool { self.words.iter().any(|item| item.is_noun()) }
    pub fn has_ending(&mut self) -> bool { self.words.iter().any(|item| item.is_ending()) }
    pub fn has_verb(&mut self) -> bool { self.words.iter().any(|item| item.is_verb()) }
    pub fn has_and(&mut self) -> bool { self.words.iter().any(|item| item.is_and()) }

    pub fn expect_noun(&mut self, he_she_it_override: Option<bool>) {
        let mut nouns = indexes!(self.words.iter_mut().enumerate().filter(|&(_, ref word)| word.is_noun())).collect();
        let mut sample = self.sample(&mut nouns);
        if let Some(new_he_she_it) = he_she_it_override {
            if let Word::Noun(ref mut he_she_it, _) = sample {
                *he_she_it = new_he_she_it;
            }
        }
        self.completed.push(sample);

        if self.rand.gen() && self.has_and() && self.has_noun() {
            self.expect_and();
            self.expect_noun(Some(false));
        }
    }
    pub fn expect_ending(&mut self) {
        let mut endings = indexes!(self.words.iter_mut().enumerate().filter(|&(_, ref word)| word.is_ending())).collect();
        let sample = self.sample(&mut endings);
        self.completed.push(sample);
    }
    pub fn expect_verb(&mut self) {
        let mut verbs = indexes!(self.words.iter_mut().enumerate().filter(|&(_, ref word)| word.is_verb())).collect();
        let verb = self.sample(&mut verbs);
        let noun = match verb {
            Word::Verb(Verb(noun, _)) => noun,
            _ => unreachable!()
        };
        self.completed.push(verb);
        if noun {
            if self.has_noun() {
                self.expect_noun(None);
            } else {
                self.completed.push(Word::Unfinished);
            }
        }
    }
    pub fn expect_and(&mut self) {
        let pos = self.words.iter().position(|word| word.is_and());
        self.completed.push(self.words.remove(pos.unwrap()));
    }
}
impl Display for Generator {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut he_she_it = false;
        let mut first = true;
        // 128 is just a guess.
        for word in &self.completed {
            if word.is_ending() {
                write!(f, ",")?;
            }
            if !first && !word.is_unfinished() {
                write!(f, " ")?;
            }
            first = false;
            match *word {
                Word::Noun(new_he_she_it, ref noun) => {
                    he_she_it = new_he_she_it;
                    write!(f, "{}", noun)
                },
                Word::Verb(ref verb) => {
                    write!(f, "{}", verb.gen(he_she_it))
                },
                Word::Ending(ref ending) => write!(f, "{}", ending),
                Word::And => write!(f, "and"),
                Word::Unfinished => write!(f, "... eh... uhnn...")
            }?;
        }
        let last = self.completed.last();
        if let Some(last) = last {
            if !last.is_ending() && !last.is_unfinished() {
                write!(f, "!")?;
            }
        }

        Ok(())
    }
}
