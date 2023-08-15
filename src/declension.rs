pub fn decline<'a>(word: String) -> (String, Vec<(&'a str, &'a str, &'a str)>) {
    let declension_patterns = [
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
                ("Illative", "an", "uosna"),
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
