use WordsFile;

#[derive(Debug, Fail)]
pub enum WordsFileCorrupt {
    #[fail(display = "Corrupt words file! Line {} in \"{}\" doesn't have boolean prefix.", _1, _0)]
    Corrupt(&'static str, usize),
    #[fail(display = "Corrupt words file! File {} is empty!", _0)]
    EmptyFile(&'static str)
}

pub fn parse_file(name: &'static str, content: &str) -> Result<Vec<(bool, String)>, WordsFileCorrupt> {
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

pub fn open_default() -> WordsFile {
    WordsFile {
        nouns: parse_file("nouns", include_str!("words/nouns")).expect("built-in words file is broken"),
        endings: parse_file("endings", include_str!("words/endings")).expect("built-in words file is broken"),
        verbs: parse_file("verbs", include_str!("words/verbs")).expect("built-in words file is broken")
    }
}
