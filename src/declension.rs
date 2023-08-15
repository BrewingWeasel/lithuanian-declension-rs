pub fn decline<'a>(word: String) -> (String, Vec<(&'a str, &'a str, &'a str)>) {
    let declension_patterns = [
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
    let mut declension = Vec::new();
    for (ending, _declension_name, declensions) in declension_patterns {
        if word.ends_with(ending) {
            let stem = word.strip_suffix(ending).unwrap_or(&word).to_owned();
            for (cur_declension, ending_sing, ending_plur) in declensions {
                declension.push((cur_declension, ending_sing, ending_plur));
            }
            return (stem, declension);
        }
    }
    (word, declension)
}
