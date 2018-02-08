#[derive(Debug, Clone, PartialEq, Eq)]
/// Something they do, like "farted on"
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
/// Not really a word, more like a substring that makes up a sentence.
pub enum Word {
    /// Someone/something, like "your math teacher"
    Noun(bool, String),
    /// An ending, like ", and you know it's true!"
    Ending(String),
    /// Something they do, like "farted on"
    Verb(Verb),
    /// "and"
    And,
    /// When the generator runs out of words, creates "... eh... uhnn..."
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
    pub fn is_unfinished(&self) -> bool {
        if let Word::Unfinished = *self {
            return true;
        }
        false
    }
}
