extern crate rand;

mod words;

use rand::Rng;
use std::io;
use std::io::Write;
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
	let mut rand = rand::thread_rng();
	let mut words = Vec::new();

	let mut num_nouns = 0;
	let mut num_verbs = 0;

	while words.len() < AMOUNT as usize {
		if num_nouns < MAX && rand.gen() {
			num_nouns += 1;
			words.push(gen_noun(&mut rand));
		} else if num_verbs < MAX && rand.gen() {
			num_verbs += 1;
			words.push(gen_verb(&mut rand));
		} else if num_nouns >= MIN && num_verbs >= MIN {
			match rand.gen::<u8>() % 10 {
				0 => words.push(gen_ending(&mut rand)),
				1 => words.push(Word::And),
				_ => {},
			}
		}
	}

	let mut last = None;
	let mut he_she_it = false;
	let mut finished = false;
	let mut space = "";

	for _ in 0..50 {
		if words.is_empty() {
			finished = true;
			break;
		}

		let i = rand.gen::<usize>() % words.len();
		let word = words[i].clone();

		match word {
			Word::Noun(ref string, new_he_she_it) => {
				if last.is_some() {
					if let Word::And = *last.as_ref().unwrap() {
					} else {
						continue;
					}
				}

				last = Some(word.clone());
				words.remove(i);
				print!("{}{}", space, string);
				if space.is_empty() {
					space = " ";
				}
				he_she_it = new_he_she_it;
			},
			Word::Verb(ref verb) => {
				if last.is_none() {
					continue;
				}
				if let Word::Verb(_) = *last.as_ref().unwrap() {
					continue;
				}

				let mut noun = None;
				let mut new_he_she_it = he_she_it;

				if verb.has_noun() {
					let item = words
						.iter()
						.find(
							|item| if let Word::Noun(..) = **item {
								true
							} else {
								false
							}
						);
					if let Some(item) = item {
						if let Word::Noun(ref string, new_he_she_it2) = *item {
							new_he_she_it = new_he_she_it2;
							noun = Some(string.clone());
						} else {
							unreachable!();
						}
					} else {
						continue;
					}
				}

				last = Some(word.clone());
				words.remove(i);
				print!(
					"{}{}",
					space,
					verb.gen(he_she_it, noun.as_ref().map(|string| string.as_str()))
				);
				if space.is_empty() {
					space = " ";
				}
				he_she_it = new_he_she_it;

				if !words
				        .iter()
				        .any(|item| if let Word::And = *item { true } else { false }) {
					finished = true;
					break;
				}
			},
			Word::Ending(ref ending) => {
				if last.is_none() {
					continue;
				}
				if let Word::Verb(_) = *last.as_ref().unwrap() {
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
				if last.is_none() {
					continue;
				}
				if let Word::And = *last.as_ref().unwrap() {
					continue;
				}

				last = Some(word.clone());
				words.remove(i);
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
		println!("{:?}", words);
		writeln!(io::stderr(), "Too many tries. Stuck in infinite loop?").unwrap();
		1
	}
}
