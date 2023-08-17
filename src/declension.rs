static DECLENSION_PATTERNS: [(&str, &str, [(&str, &str, &str); 8]); 7] = [
    (
        "ias",
        "first declension",
        [
            ("Nominative", "ias", "iai"),
            ("Genitive", "io", "ių"),
            ("Dative", "iui", "iams"),
            ("Accusative", "ią", "ius"),
            ("Instrumental", "iu", "iais"),
            ("Locative", "yje", "iuose"),
            ("Vocative", "y", "iai"),
            ("Illative", "in", "iuosna"),
        ],
    ),
    (
        "as",
        "first declension",
        [
            ("Nominative", "as", "ai"),
            ("Genitive", "o", "ų"),
            ("Dative", "ui", "ams"),
            ("Accusative", "ą", "us"),
            ("Instrumental", "u", "ais"),
            ("Locative", "e", "uose"),
            ("Vocative", "ai", "ai"),
            ("Illative", "an", "uosna"),
        ],
    ),
    (
        "a",
        "second declension",
        [
            ("Nominative", "a", "os"),
            ("Genitive", "os", "ų"),
            ("Dative", "ai", "oms"),
            ("Accusative", "ą", "as"),
            ("Instrumental", "a", "omis"),
            ("Locative", "oje", "ose"),
            ("Vocative", "a", "os"),
            ("Illative", "on", "osna"),
        ],
    ),
    (
        "ė",
        "second declension",
        [
            ("Nominative", "ė", "ės"),
            ("Genitive", "ės", "ių"),
            ("Dative", "ei", "ėms"),
            ("Accusative", "ę", "es"),
            ("Instrumental", "e", "ėmis"),
            ("Locative", "ėje", "ėse"),
            ("Vocative", "e", "ės"),
            ("Illative", "ėn", "ėsna"),
        ],
    ),
    (
        "ius",
        "fourth declension",
        [
            ("Nominative", "ius", "iai"),
            ("Genitive", "iaus", "ių"),
            ("Dative", "iui", "iams"),
            ("Accusative", "ių", "ius"),
            ("Instrumental", "iumi", "iais"),
            ("Locative", "iuje", "iuose"),
            ("Vocative", "iau", "iai"),
            ("Illative", "iun", "?"),
        ],
    ),
    (
        "us",
        "fourth declension",
        [
            ("Nominative", "us", "ūs"),
            ("Genitive", "aus", "ų"),
            ("Dative", "ui", "ums"),
            ("Accusative", "ų", "us"),
            ("Instrumental", "umi", "umis"),
            ("Locative", "uje", "uose"),
            ("Vocative", "au", "ūs"),
            ("Illative", "un", "?"),
        ],
    ),
    (
        "uo",
        "fifth declension",
        [
            ("Nominative", "uo", "enys"),
            ("Genitive", "ens", "enų"),
            ("Dative", "eniui", "enums"),
            ("Accusative", "enį", "enus"),
            ("Instrumental", "umi", "umis"),
            ("Locative", "uje", "uose"),
            ("Vocative", "au", "ūs"),
            ("Illative", "not implemented", "not implemented"),
        ],
    ),
];

static IS_1: (&str, &str, [(&str, &str, &str); 8]) = (
    "is",
    "first declension",
    [
        ("Nominative", "is", "iai"),
        ("Genitive", "io", "ių"),
        ("Dative", "iui", "iams"),
        ("Accusative", "į", "ius"),
        ("Instrumental", "iu", "iais"),
        ("Locative", "yje", "iuose"),
        ("Vocative", "i", "iai"),
        ("Illative", "in", "not implemented"),
    ],
);

static IS_3M: (&str, &str, [(&str, &str, &str); 8]) = (
    "is",
    "third declension",
    [
        ("Nominative", "is", "ys"),
        ("Genitive", "ies", "ių"), // TODO: not always ių
        ("Dative", "iai", "ims"),
        ("Accusative", "į", "is"),
        ("Instrumental", "imi", "imis"),
        ("Locative", "yje", "yse"),
        ("Vocative", "ie", "ys"),
        ("Illative", "in", "not implemented"),
    ],
);

static IS_3F: (&str, &str, [(&str, &str, &str); 8]) = (
    "is",
    "third declension",
    [
        ("Nominative", "is", "ys"),
        ("Genitive", "ies", "ių"), // TODO: not always ių
        ("Dative", "iui", "ims"),
        ("Accusative", "į", "is"),
        ("Instrumental", "imi", "imis"),
        ("Locative", "yje", "yse"),
        ("Vocative", "ie", "ys"),
        ("Illative", "in", "not implemented"),
    ],
);

static IS_FIRST_DECL: [&str; 1] = ["slėgis"];

static IS_THIRD_MASC: [&str; 1] = ["akis"];

static IS_THIRD_FEM: [&str; 1] = ["vagis"];

pub fn decline<'a>(word: String) -> Result<(String, [(&'a str, &'a str, &'a str); 8]), String> {
    for (ending, _declension_name, declensions) in DECLENSION_PATTERNS {
        if word.ends_with(ending) {
            let stem = word.strip_suffix(ending).unwrap_or(&word).to_owned();
            return Ok((stem, declensions));
        }
    }
    if word.ends_with("is") {
        let stem = word.strip_suffix("is").unwrap_or(&word).to_owned();
        if IS_FIRST_DECL.contains(&word.as_str()) {
            return Ok((stem, IS_1.2));
        } else if IS_THIRD_MASC.contains(&word.as_str()) {
            return Ok((stem, IS_3M.2));
        } else if IS_THIRD_FEM.contains(&word.as_str()) {
            return Ok((stem, IS_3F.2));
        }
    }
    Err("does not exit".to_owned())
}
