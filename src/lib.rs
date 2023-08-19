use declension::decline_noun;
use declension_adjective::decline_adjective;

mod declension;
mod declension_adjective;

#[derive(Debug)]
pub struct WordParts<'a> {
    pub unmodified_stem: &'a str,
    pub modified_stem: &'a str,
    pub ending: &'a str,
}

pub enum Declension<'a> {
    Noun(String, Vec<(&'a str, [WordParts<'a>; 2])>),
    Adjective(String, Vec<(&'a str, [[WordParts<'a>; 2]; 2])>),
}

pub fn decline<'a>(word: String) -> Result<Declension<'a>, String> {
    if word == "įdomus" {
        let (stem, declension) = decline_adjective(word)?;
        Ok(Declension::Adjective(stem, declension))
    } else {
        let (stem, declension) = decline_noun(word)?;
        Ok(Declension::Noun(stem, declension))
    }
}

static PALATIZE_ENDINGS: [&str; 4] = ["io", "iu", "ia", "ių"];

pub fn create_ending<'a>(
    orig: &'a str,
    original_char: &'a str,
    new: &'a str,
) -> (&'a str, &'a str, &'a str) {
    for i in &PALATIZE_ENDINGS {
        if orig.starts_with(i) {
            return ("", new, orig);
        }
    }
    (original_char, "", orig)
}
