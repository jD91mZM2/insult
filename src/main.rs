// All words are taken from
// http://ohsir-the-insult-simulator.wikia.com/wiki/All_of_the_words_in_Oh...
// Sir!!_The_Insult_Simulator
//
// What is referred to as "word" is actually not a word by the way.
// It's a part of a sentence.

extern crate failure;
extern crate insult;
extern crate rand;
extern crate xdg;

use failure::Error;
use insult::{parse_file, WordsFile};
use std::{
    borrow::Cow,
    {fs, io, process}
};
use xdg::BaseDirectories;

fn read_file(xdg: &BaseDirectories, name: &str, default: &'static str)
    -> Result<Cow<'static, str>, io::Error>
{
    if let Some(config) = xdg.find_config_file(name) {
        let mut buf = fs::read_to_string(config)?;
        Ok(Cow::from(buf))
    } else {
        fs::write(xdg.place_config_file(name)?, default)?;
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
            parse_file($file, &*read_file(&xdg, $file, include_str!(concat!("words/", $file)))?)?
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
