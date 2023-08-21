// vi: fdm=marker
use crate::{create_ending, WordParts};

//{{{
static ADJECTIVE_DECLENSION_PATTERNS: [(&str, &str, [[&str; 5]; 6]); 4] = [
    (
        "as",
        "first declension",
        [
            ["Nominative", "as", "a", "i", "os"],
            ["Genitive", "o", "os", "ų", "ų"],
            ["Dative", "am", "ai", "iems", "oms"],
            ["Accusative", "ą", "ą", "us", "as"],
            ["Instrumental", "u", "a", "ais", "omis"],
            ["Locative", "ame", "oje", "uose", "ose"],
        ],
    ),
    (
        "ias",
        "first declension",
        [
            ["Nominative", "ias", "ia", "i", "ios"],
            ["Genitive", "io", "ios", "ių", "ių"],
            ["Dative", "iam", "iai", "iems", "ioms"],
            ["Accusative", "ią", "ią", "ius", "ias"],
            ["Instrumental", "iu", "ia", "iais", "iomis"],
            ["Locative", "iame", "ioje", "iuose", "iose"],
        ],
    ),
    (
        "is",
        "first declension",
        [
            ["Nominative", "is", "ė", "i", "ės"],
            ["Genitive", "io", "ės", "ių", "ių"],
            ["Dative", "iam", "ei", "iems", "ėms"],
            ["Accusative", "į", "ę", "ius", "es"],
            ["Instrumental", "iu", "e", "iais", "ėmis"],
            ["Locative", "iame", "ėje", "iuose", "ėse"],
        ],
    ),
    (
        "us",
        "second declension",
        [
            ["Nominative", "us", "i", "ūs", "ios"],
            ["Genitive", "aus", "ios", "ių", "ių"],
            ["Dative", "iam", "iai", "iems", "ioms"],
            ["Accusative", "ų", "ią", "ius", "ias"],
            ["Instrumental", "iu", "ia", "iais", "iomis"],
            ["Locative", "iame", "ioje", "iuose", "iose"],
        ],
    ),
];
//}}}

pub fn decline_adjective<'a>(
    word: String,
) -> Result<(String, Vec<(&'a str, [[WordParts<'a>; 2]; 2])>), String> {
    for (ending, _declension_name, declensions) in ADJECTIVE_DECLENSION_PATTERNS {
        if word.ends_with(ending) {
            let stem = word.strip_suffix(ending).unwrap_or(&word).to_owned();
            return Ok(parse_declensions(stem, declensions));
        }
    }
    Err("does not exit".to_owned())
}

fn parse_declensions(
    mut stem: String,
    declension: [[&str; 5]; 6],
) -> (String, Vec<(&str, [[WordParts<'_>; 2]; 2])>) {
    if stem.ends_with('d') {
        stem.pop();
        (stem, handle_substitutions("d", "dž", declension))
    } else if stem.ends_with('t') {
        stem.pop();
        (stem, handle_substitutions("t", "č", declension))
    } else if declension[0][2].starts_with("en") {
        (stem, handle_substitutions("", "", declension))
    } else {
        (stem, declension.map(create_nonexistent_prefixes).into())
    }
}

fn create_nonexistent_prefixes(declension: [&str; 5]) -> (&str, [[WordParts<'_>; 2]; 2]) {
    (
        declension[0],
        [
            [
                WordParts {
                    unmodified_stem: "",
                    modified_stem: "",
                    ending: declension[1],
                },
                WordParts {
                    unmodified_stem: "",
                    modified_stem: "",
                    ending: declension[2],
                },
            ],
            [
                WordParts {
                    unmodified_stem: "",
                    modified_stem: "",
                    ending: declension[3],
                },
                WordParts {
                    unmodified_stem: "",
                    modified_stem: "",
                    ending: declension[4],
                },
            ],
        ],
    )
}

fn handle_substitutions<'a>(
    original: &'a str,
    new: &'a str,
    declension: [[&'a str; 5]; 6],
) -> Vec<(&'a str, [[WordParts<'a>; 2]; 2])> {
    let mut values = Vec::new();

    for [name, singm, singf, plurm, plurf] in declension {
        let (sing_orig, sing_prefix, sing) = create_ending(singm, original, new);
        let (plur_orig, plur_prefix, plur) = create_ending(plurm, original, new);
        let (sing_origf, sing_prefixf, singf) = create_ending(singf, original, new);
        let (plur_origf, plur_prefixf, plurf) = create_ending(plurf, original, new);
        values.push((
            name,
            [
                [
                    WordParts {
                        unmodified_stem: sing_orig,
                        modified_stem: sing_prefix,
                        ending: sing,
                    },
                    WordParts {
                        unmodified_stem: sing_origf,
                        modified_stem: sing_prefixf,
                        ending: singf,
                    },
                ],
                [
                    WordParts {
                        unmodified_stem: plur_orig,
                        modified_stem: plur_prefix,
                        ending: plur,
                    },
                    WordParts {
                        unmodified_stem: plur_origf,
                        modified_stem: plur_prefixf,
                        ending: plurf,
                    },
                ],
            ],
        ))
    }
    values
}
