// All words are taken from
// http://ohsir-the-insult-simulator.wikia.com/wiki/All_of_the_words_in_Oh...
// Sir!!_The_Insult_Simulator
//
// What is referred to as "word" is actually not a word by the way.
// It's a part of a sentence.

#[macro_use] extern crate failure;
extern crate insult;
extern crate rand;
extern crate xdg;

use failure::Error;
use insult::WordsFile;
use std::borrow::Cow;
use std::process;
use std::fs::File;
use std::io::{self, Read, Write};
use xdg::BaseDirectories;

#[derive(Debug, Fail)]
enum WordsFileCorrupt {
    #[fail(display = "Corrupt words file! Line {} in \"{}\" doesn't have boolean prefix.", _1, _0)]
    Corrupt(&'static str, usize),
    #[fail(display = "Corrupt words file! File {} is empty!", _0)]
    EmptyFile(&'static str)
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
fn read_file(xdg: &BaseDirectories, name: &str, default: &'static str)
    -> Result<Cow<'static, str>, io::Error>
{
    if let Some(config) = xdg.find_config_file(name) {
        let mut buf = String::new();
        File::open(config)?.read_to_string(&mut buf)?;
        Ok(Cow::from(buf))
    } else {
        File::create(xdg.place_config_file(name)?)?.write_all(default.as_bytes())?;
        Ok(Cow::from(default))
    }
}

fn main() {
    let code = if let Err(err) = do_main() {
        eprintln!("{}", err);
        1
    } else { 0 };
    process::exit(code);
}
fn do_main() -> Result<(), Error> {
    let xdg = BaseDirectories::with_prefix("insult")?;

    macro_rules! read_file {
        ($file:expr) => {
            parse_file($file, &*read_file(&xdg, $file, include_str!($file))?)?
        }
    }

    let words = WordsFile {
        nouns: read_file!("nouns"),
        endings: read_file!("endings"),
        verbs: read_file!("verbs")
    };
    println!("{}", words.generate(rand::thread_rng()));
    Ok(())
}
