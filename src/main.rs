// All words are taken from
// http://ohsir-the-insult-simulator.wikia.com/wiki/All_of_the_words_in_Oh...
// Sir!!_The_Insult_Simulator
//
// What is referred to as "word" is actually not a word by the way.
// It's a part of a sentence.

extern crate rand;

mod words;
mod config;

use rand::Rng;
use std::process;
use words::*;

const AMOUNT: u8 = 10;
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
	let wordsfile = match WordsFile::parse_touple(wordstuple) {
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

	let mut completed = Vec::new();
	let mut he_she_it = false;
	let mut finished = false;
	let mut space = "";

	for _ in 0..100 {
		if words.is_empty() {
			finished = true;
			break;
		}

		let mut i = rand.gen::<usize>() % words.len();
		let word = words[i].clone();

		match word {
			Word::Noun(new_he_she_it, ref string) => {
				if let Some(last) = completed.last() {
					if let Word::And = *last {
					} else {
						continue;
					}
				}

				he_she_it = if completed.len() >= 2 {
                    if let Word::And = completed[completed.len() - 1] {
                        if let Word::Noun(..) = completed[completed.len() - 2] {
                            false
                        } else {
                            new_he_she_it
                        }
                    } else {
                        new_he_she_it
                    }
                } else {
                    new_he_she_it
                };

				completed.push(words.remove(i));
				print!("{}{}", space, string);
				if space.is_empty() {
					space = " ";
				}
			},
			Word::Verb(ref verb) => {
				{
					let last = completed.last();
					if completed.is_empty() {
						continue;
					}
					if let Word::Noun(..) = *last.unwrap() {
					} else {
						continue;
					}
				}

				let mut noun = None;
				let mut new_he_she_it = he_she_it;

				if verb.has_noun() {
					let pos = words.iter().position(|item| if let Word::Noun(..) = *item {
						true
					} else {
						false
					});
					if let Some(pos) = pos {
						if let Word::Noun(new_he_she_it2, ref string) = words[pos] {
							new_he_she_it = new_he_she_it2;
							noun = Some(string.clone());
						} else {
							unreachable!();
						}
						words.remove(pos);
						if pos < i {
							// Uh oh! We wouldn't want words.remove(i)
							// to use an outdated index!
							i -= 1;
						}
					} else {
						continue;
					}
				}

				completed.push(words.remove(i));
				print!(
					"{}{}",
					space,
					verb.gen(he_she_it, noun.as_ref().map(|string| string.as_str()))
				);
				if space.is_empty() {
					space = " ";
				}
				he_she_it = new_he_she_it;

				if !words.iter().any(|item| if let Word::And = *item {
					true
				} else if let Word::Ending(_) = *item {
					true
				} else {
					false
				})
				{
					finished = true;
					break;
				}
			},
			Word::Ending(ref ending) => {
				let last = completed.last();
				if last.is_none() {
					continue;
				}
				if let Word::Verb(_) = *last.unwrap() {
				} else {
					continue;
				}
				if rand.gen() {
					continue;
				}

				finished = true;
				print!(", {}", ending);
				break;
			},
			Word::And => {
				{
					let last = completed.last();
					if last.is_none() {
						continue;
					}
					if let Word::And = *last.unwrap() {
						continue;
					}
				}

				completed.push(words.remove(i));
				print!("{}and", space);
				if space.is_empty() {
					space = " ";
				}
			},
		}
	}

	if finished {
		println!();
		0
	} else {
		println!();
		eprintln!("Too many tries. Stuck in infinite loop?");
		1
	}
}
