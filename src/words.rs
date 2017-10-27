use rand::{Rng, ThreadRng};
use std::ascii::AsciiExt;
use std::error::Error;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Verb(pub bool, pub String);
impl Verb {
    pub fn gen(&self, he_she_it: bool) -> String {
        // Consider making custom replace function for efficiency

        let string = if he_she_it {
            self.1
                .replace("[is/are]", "is")
                .replace("[has/have]", "has")
                .replace("[was/were]", "was")
                .replace("(s)", "s")
                .replace("(es)", "es")
        } else {
            self.1
                .replace("[is/are]", "are")
                .replace("[has/have]", "have")
                .replace("[was/were]", "were")
                .replace("(s)", "")
                .replace("(es)", "")
        };
        string
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Word {
    Noun(bool, String),
    Ending(String),
    Verb(Verb),
    And,
    Unfinished
}

impl Word {
    pub fn is_noun(&self) -> bool {
        if let Word::Noun(..) = *self {
            return true;
        }
        false
    }
    pub fn is_ending(&self) -> bool {
        if let Word::Ending(_) = *self {
            return true;
        }
        false
    }
    pub fn is_verb(&self) -> bool {
        if let Word::Verb(_) = *self {
            return true;
        }
        false
    }
    pub fn is_and(&self) -> bool {
        if let Word::And = *self {
            return true;
        }
        false
    }
}

#[derive(Debug)]
pub enum WordsFileCorrupt {
    Corrupt(&'static str, usize),
    EmptyFile(&'static str)
}

impl Error for WordsFileCorrupt {
    fn description(&self) -> &str { "Corrupt words file" }
}
impl fmt::Display for WordsFileCorrupt {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            WordsFileCorrupt::Corrupt(file, i) => {
                write!(f, "Corrupt words file! Line {} in {} doesn't have boolean prefix.", file, i)
            },
            WordsFileCorrupt::EmptyFile(name) => write!(f, "Corrupt words file! File {} is empty!", name),
        }
    }
}

pub struct WordsFile {
    nouns:   Vec<(bool, String)>,
    endings: Vec<(bool, String)>,
    verbs:   Vec<(bool, String)>
}
impl WordsFile {
    pub fn parse_tuple(tuple: (String, String, String)) -> Result<WordsFile, WordsFileCorrupt> {
        // Let's not worry about reserving since it prevents frequent reallocations
        // anyways.

        let nouns   = WordsFile::parse_file("endings", &tuple.0)?;
        let endings = WordsFile::parse_file("endings", &tuple.1)?;
        let verbs   = WordsFile::parse_file("verbs",   &tuple.2)?;

        Ok(WordsFile {
            nouns: nouns,
            endings: endings,
            verbs: verbs
        })
    }
    fn parse_file(name: &'static str, content: &str) -> Result<Vec<(bool, String)>, WordsFileCorrupt> {
        let mut lines = Vec::new();
        for (i, line) in content.lines().enumerate() {
            if line.is_empty() || line.starts_with("#") {
                continue;
            }
            let mut parts = line.splitn(2, ",").map(|item| item.trim());
            let line = match (parts.next(), parts.next()) {
                (Some(flag), Some(line)) => (if flag.eq_ignore_ascii_case("true") {
                        true
                    } else if flag.eq_ignore_ascii_case("false") {
                        false
                    } else {
                        return Err(WordsFileCorrupt::Corrupt(name, i))
                    }, line.to_string()),
                (_, _) => {
                    return Err(WordsFileCorrupt::Corrupt(name, i));
                }
            };
            lines.push(line);
        }

        if lines.is_empty() {
            return Err(WordsFileCorrupt::EmptyFile(name));
        }
        Ok(lines)
    }

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
}
