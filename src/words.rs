use rand::{Rng, ThreadRng};
use std::ascii::AsciiExt;
use std::error::Error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Verb(String);
impl Verb {
	pub fn has_noun(&self) -> bool { self.0.contains("(noun)") }
	pub fn gen(&self, he_she_it: bool, noun: Option<&str>) -> String {
		// Consider making custom replace function for efficiency

		let mut string = if he_she_it {
			self.0
				.replace("[is/are]", "is")
				.replace("[has/have]", "has")
				.replace("[was/were]", "was")
				.replace("(s)", "s")
				.replace("(es)", "es")
		} else {
			self.0
				.replace("[is/are]", "are")
				.replace("[has/have]", "have")
				.replace("[was/were]", "were")
				.replace("(s)", "")
				.replace("(es)", "")
		};

		if let Some(noun) = noun {
			string = string.replace("(noun)", noun);
		}
		string
	}
}

#[derive(Debug, Clone)]
pub enum Word {
	Noun(bool, String),
	Ending(String),
	Verb(Verb),
	And
}

#[derive(Debug)]
pub enum WordsFileCorrupt {
	NounsFileCorrupt(usize),
	EmptyFile(&'static str)
}

impl Error for WordsFileCorrupt {
	fn description(&self) -> &str { "Corrupt words file" }
}
impl fmt::Display for WordsFileCorrupt {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match *self {
			WordsFileCorrupt::NounsFileCorrupt(i) => {
				write!(
					f,
					"Corrupt words file! Line {} in nouns doesn't have boolean prefix.",
					i
				)
			},
			WordsFileCorrupt::EmptyFile(name) => write!(f, "Corrupt words file! File {} is empty!", name),
		}
	}
}

pub struct WordsFile {
	nouns: Vec<(bool, String)>,
	endings: Vec<String>,
	verbs: Vec<String>
}
impl WordsFile {
	pub fn parse_touple(touple: (String, String, String)) -> Result<WordsFile, WordsFileCorrupt> {
		// Let's not worry about reserving since it prevents frequent reallocations
		// anyways.
		let mut nouns = Vec::new();
		for (i, line) in touple.0.lines().enumerate() {
			if line.is_empty() || line.starts_with("#") {
				continue;
			}

			let parts: Vec<&str> = line.splitn(2, ", ").collect();
			if parts.len() != 2 {
				return Err(WordsFileCorrupt::NounsFileCorrupt(i));
			}

			if parts[0].eq_ignore_ascii_case("true") {
				nouns.push((true, parts[1].to_string()));
			} else if parts[0].eq_ignore_ascii_case("false") {
				nouns.push((false, parts[1].to_string()));
			} else {
				return Err(WordsFileCorrupt::NounsFileCorrupt(i));
			}
		}

		if nouns.is_empty() {
			return Err(WordsFileCorrupt::EmptyFile("nouns"));
		}

		macro_rules! parse_string {
			($index:tt, $name:expr) => {
				{
					let mut lines = Vec::new();
					for line in touple.$index.lines() {
						if line.is_empty() || line.starts_with("#") {
							continue;
						}
						lines.push(line.to_string());
					}

					if lines.is_empty() {
						return Err(WordsFileCorrupt::EmptyFile($name));
					}
					lines
				}
			}
		}
		let endings = parse_string!(1, "endings");
		let verbs = parse_string!(2, "verbs");

		Ok(WordsFile {
			nouns: nouns,
			endings: endings,
			verbs: verbs
		})
	}

	pub fn gen_noun(&self, rand: &mut ThreadRng) -> Word {
		let &(he_she_it, ref word) = &self.nouns[rand.gen::<usize>() % self.nouns.len()];
		Word::Noun(he_she_it, word.clone())
	}
	pub fn gen_ending(&self, rand: &mut ThreadRng) -> Word {
		let word = &self.endings[rand.gen::<usize>() % self.endings.len()];
		Word::Ending(word.clone())
	}
	pub fn gen_verb(&self, rand: &mut ThreadRng) -> Word {
		let word = &self.verbs[rand.gen::<usize>() % self.verbs.len()];
		Word::Verb(Verb(word.clone()))
	}
}
