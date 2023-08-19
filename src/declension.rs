// vi: fdm=marker

use crate::{create_ending, WordParts};

//{{{
static DECLENSION_PATTERNS: [(&str, &str, [[&str; 3]; 8]); 7] = [
    (
        "ias",
        "first declension",
        [
            ["Nominative", "ias", "iai"],
            ["Genitive", "io", "ių"],
            ["Dative", "iui", "iams"],
            ["Accusative", "ią", "ius"],
            ["Instrumental", "iu", "iais"],
            ["Locative", "yje", "iuose"],
            ["Vocative", "y", "iai"],
            ["Illative", "in", "iuosna"],
        ],
    ),
    (
        "as",
        "first declension",
        [
            ["Nominative", "as", "ai"],
            ["Genitive", "o", "ų"],
            ["Dative", "ui", "ams"],
            ["Accusative", "ą", "us"],
            ["Instrumental", "u", "ais"],
            ["Locative", "e", "uose"],
            ["Vocative", "ai", "ai"], // TODO: vocative for as changes
            ["Illative", "an", "uosna"],
        ],
    ),
    (
        "a",
        "second declension",
        [
            ["Nominative", "a", "os"],
            ["Genitive", "os", "ų"],
            ["Dative", "ai", "oms"],
            ["Accusative", "ą", "as"],
            ["Instrumental", "a", "omis"],
            ["Locative", "oje", "ose"],
            ["Vocative", "a", "os"],
            ["Illative", "on", "osna"],
        ],
    ),
    (
        "ė",
        "second declension",
        [
            ["Nominative", "ė", "ės"],
            ["Genitive", "ės", "ių"],
            ["Dative", "ei", "ėms"],
            ["Accusative", "ę", "es"],
            ["Instrumental", "e", "ėmis"],
            ["Locative", "ėje", "ėse"],
            ["Vocative", "e", "ės"],
            ["Illative", "ėn", "ėsna"],
        ],
    ),
    (
        "ius",
        "fourth declension",
        [
            ["Nominative", "ius", "iai"],
            ["Genitive", "iaus", "ių"],
            ["Dative", "iui", "iams"],
            ["Accusative", "ių", "ius"],
            ["Instrumental", "iumi", "iais"],
            ["Locative", "iuje", "iuose"],
            ["Vocative", "iau", "iai"],
            ["Illative", "iun", "?"],
        ],
    ),
    (
        "us",
        "fourth declension",
        [
            ["Nominative", "us", "ūs"],
            ["Genitive", "aus", "ų"],
            ["Dative", "ui", "ums"],
            ["Accusative", "ų", "us"],
            ["Instrumental", "umi", "umis"],
            ["Locative", "uje", "uose"],
            ["Vocative", "au", "ūs"],
            ["Illative", "un", "?"],
        ],
    ),
    (
        "uo",
        "fifth declension",
        [
            ["Nominative", "uo", "enys"],
            ["Genitive", "ens", "enų"],
            ["Dative", "eniui", "enums"],
            ["Accusative", "enį", "enus"],
            ["Instrumental", "eniu", "enimis"],
            ["Locative", "enyje", "enyse"],
            ["Vocative", "enie", "enys"],
            ["Illative", "?", "?"],
        ],
    ),
];
//}}}

//{{{
static IS_1: (&str, &str, [[&str; 3]; 8]) = (
    "is",
    "first declension",
    [
        ["Nominative", "is", "iai"],
        ["Genitive", "io", "ių"],
        ["Dative", "iui", "iams"],
        ["Accusative", "į", "ius"],
        ["Instrumental", "iu", "iais"],
        ["Locative", "yje", "iuose"],
        ["Vocative", "i", "iai"],
        ["Illative", "in", "?"],
    ],
);

static IS_3M: (&str, &str, [[&str; 3]; 8]) = (
    "is",
    "third declension",
    [
        ["Nominative", "is", "ys"],
        ["Genitive", "ies", "ių"], // TODO: not always ių
        ["Dative", "iai", "ims"],
        ["Accusative", "į", "is"],
        ["Instrumental", "imi", "imis"],
        ["Locative", "yje", "yse"],
        ["Vocative", "ie", "ys"],
        ["Illative", "in", "?"],
    ],
);

static IS_3F: (&str, &str, [[&str; 3]; 8]) = (
    "is",
    "third declension",
    [
        ["Nominative", "is", "ys"],
        ["Genitive", "ies", "ių"], // TODO: not always ių
        ["Dative", "iui", "ims"],
        ["Accusative", "į", "is"],
        ["Instrumental", "imi", "imis"],
        ["Locative", "yje", "yse"],
        ["Vocative", "ie", "ys"],
        ["Illative", "in", "?"],
    ],
);
//}}}

//{{{
// automatically generated; see https://github.com/BrewingWeasel/lithuanian_morph_data
static IS_FIRST_DECL: [&str; 661] = [
    "aistruolis",
    "akimirksnis",
    "akmenėlis",
    "alytiškis",
    "alkis",
    "alkoholis",
    "alksnis",
    "amerikietis",
    "ansamblis",
    "antgalis",
    "antkapis",
    "antplūdis",
    "antveidis",
    "anūkėlis",
    "aplinkkelis",
    "apšepėlis",
    "arkliamėšlis",
    "asmenvardis",
    "ateivis",
    "atgarsis",
    "atlygis",
    "atodrėkis",
    "atodūsis",
    "atsiskyrėlis",
    "atspalvis",
    "atvykėlis",
    "atžvilgis",
    "auklėtinis",
    "aukštaitis",
    "aukštis",
    "automobilis",
    "autsaideris",
    "avigalvis",
    "balandis",
    "balionėlis",
    "balselis",
    "baltarusis",
    "bandiklis",
    "bandžiulis",
    "banginis",
    "batelis",
    "bedarbis",
    "bendrabutis",
    "bendradarbis",
    "bendrakeleivis",
    "bendraklasis",
    "bendramintis",
    "bendražygis",
    "berniūkštis",
    "bičiulis",
    "blizgutis",
    "blyksnis",
    "blogis",
    "brokeris",
    "brokolis",
    "brolelis",
    "brolis",
    "brūkšnelis",
    "budelis",
    "buhalteris",
    "buldozeris",
    "būrelis",
    "butelis",
    "charakteris",
    "čekis",
    "černobylietis",
    "dalyvis",
    "dangtelis",
    "dangtis",
    "darbastalis",
    "darželis",
    "daugelis",
    "daugiklis",
    "daviklis",
    "dažnis",
    "deliuvis",
    "dėsnis",
    "dešimtmetis",
    "didmiestis",
    "didvyris",
    "dienoraštis",
    "dienovidis",
    "dienraštis",
    "dirgiklis",
    "dirvožemis",
    "diskelis",
    "dispečeris",
    "dydis",
    "dygis",
    "doleris",
    "drabužėlis",
    "drabužis",
    "draugužis",
    "draustinis",
    "drožiklis",
    "drugelis",
    "dubleris",
    "dujotiekis",
    "dūlėsis",
    "dūris",
    "dūžis",
    "dvariškis",
    "dvasiškis",
    "dvidešimtmetis",
    "dviratis",
    "egiptietis",
    "eilėraštis",
    "eilinis",
    "emalis",
    "erelis",
    "eteris",
    "europietis",
    "fermeris",
    "festivalis",
    "gabalėlis",
    "gailestis",
    "galvūgalis",
    "gamtovaizdis",
    "garliavietis",
    "gaubtuvėlis",
    "gediminaitis",
    "geležinkelis",
    "geltonis",
    "gėris",
    "giminaitis",
    "girtuoklis",
    "gylis",
    "gyvis",
    "gluosnis",
    "greitis",
    "greitkelis",
    "grybelis",
    "grožis",
    "gruodis",
    "gumbelis",
    "guolis",
    "gurkšnelis",
    "gurkšnis",
    "ilgis",
    "indėlis",
    "išeivis",
    "išlaikytinis",
    "išpuolis",
    "įdirbis",
    "įgaliotinis",
    "įgūdis",
    "įkarštis",
    "įlinkis",
    "įnagis",
    "įniršis",
    "įpėdinis",
    "įprotis",
    "įrankis",
    "įspūdis",
    "įsūnis",
    "įvaizdis",
    "įvartis",
    "įvykis",
    "jankis",
    "jaunalatvis",
    "jaunikis",
    "jauniklis",
    "jaunuolis",
    "jautis",
    "jėgeris",
    "jotvingis",
    "jungiklis",
    "juokelis",
    "jūreivis",
    "kailis",
    "kaimelis",
    "kaimietis",
    "kaištelis",
    "kaklelis",
    "kalnelis",
    "kalvagūbris",
    "kalvis",
    "kampelis",
    "kamputis",
    "kamštelis",
    "kamštis",
    "kancleris",
    "kareivis",
    "kariškis",
    "karštis",
    "kartėlis",
    "karvelis",
    "katarsis",
    "kateteris",
    "kaunietis",
    "kauniškis",
    "kąsnis",
    "keistuolis",
    "keitiklis",
    "keleivis",
    "kelis",
    "ketvirtadalis",
    "ketvirtadienis",
    "ketvirtis",
    "kiaušinėlis",
    "kiaušinis",
    "kiekis",
    "kilimėlis",
    "kirtis",
    "kirvis",
    "kiškis",
    "kiškutis",
    "klaipėdietis",
    "klonis",
    "koklis",
    "kolibris",
    "koliuvis",
    "kolūkis",
    "kompiuteris",
    "koncertmeisteris",
    "konteineris",
    "konvejeris",
    "kostiumėlis",
    "kraštelis",
    "kraštovaizdis",
    "kraujospūdis",
    "krepšinis",
    "krikštatėvis",
    "kryžiuotis",
    "krūmokšnis",
    "krūvis",
    "kūdikis",
    "kumpis",
    "kumštis",
    "kūnelis",
    "kunigaikštis",
    "kūrinėlis",
    "kuršis",
    "laikiklis",
    "laikotarpis",
    "laikraštis",
    "laikrodis",
    "laimikis",
    "laipsnis",
    "laiptelis",
    "laisvalaikis",
    "laivelis",
    "langelis",
    "lankstelis",
    "lapelis",
    "lapkotis",
    "lašelis",
    "latgalis",
    "latvis",
    "lazeris",
    "ledonešis",
    "lieptelis",
    "lietpaltis",
    "lietuvis",
    "liežuvėlis",
    "liežuvis",
    "ligonis",
    "likeris",
    "likutis",
    "limfmazgis",
    "lituoklis",
    "lyderis",
    "lydmetalis",
    "lygis",
    "lyvis",
    "lobis",
    "lopšelis",
    "lūkestis",
    "luotelis",
    "lūpdažis",
    "maišelis",
    "makleris",
    "mastelis",
    "mazgelis",
    "mažametis",
    "mažylis",
    "mažutėlis",
    "medalis",
    "medis",
    "medsraigtis",
    "medutis",
    "mėgintuvėlis",
    "megztinis",
    "menedžeris",
    "mėnulis",
    "metraštis",
    "miestelis",
    "miestietis",
    "miežis",
    "ministeris",
    "minkštasuolis",
    "mirksnis",
    "miškelis",
    "modelis",
    "modulis",
    "mokestis",
    "moksleivis",
    "molis",
    "molžemis",
    "motelis",
    "motorlaivis",
    "mūsiškis",
    "mūšis",
    "namelis",
    "namiškis",
    "našlaitis",
    "naugardietis",
    "naujagimis",
    "nedaugelis",
    "neišmanėlis",
    "nepilnametis",
    "netikėlis",
    "nevykėlis",
    "nulis",
    "numeris",
    "nuolydis",
    "nuopuolis",
    "nuosprendis",
    "nuostolis",
    "nuotykis",
    "nuotolis",
    "nuovargis",
    "nusidėjėlis",
    "nusikaltėlis",
    "pabėgėlis",
    "padegėlis",
    "pajūris",
    "pakaruoklis",
    "pakaušis",
    "pakelis",
    "pakinklis",
    "paladis",
    "paplotėlis",
    "papunktis",
    "parapijietis",
    "parkeris",
    "paršelis",
    "partneris",
    "pasaulėvaizdis",
    "pasaulis",
    "pasienietis",
    "pasienis",
    "pastolis",
    "pašalietis",
    "paukštelis",
    "paukštis",
    "pavasaris",
    "paveikslėlis",
    "peilis",
    "penktadalis",
    "penktadienis",
    "persikėlėlis",
    "pertrūkis",
    "piliakalnis",
    "pilietis",
    "piltuvėlis",
    "pinigėlis",
    "pirmadienis",
    "pirmavaizdis",
    "pirmeivis",
    "pyktis",
    "pjoviklis",
    "pjūvis",
    "plienagalvis",
    "plotis",
    "pobūdis",
    "pobūvis",
    "podirvis",
    "poelgis",
    "pogrindis",
    "pogrupis",
    "poilsis",
    "pojūtis",
    "pokalbis",
    "pokytis",
    "polaidis",
    "polėkis",
    "polinkis",
    "pomėgis",
    "popierėlis",
    "poreikis",
    "posakis",
    "posėdis",
    "posistemis",
    "poskyris",
    "poslinkis",
    "posūkis",
    "posūnis",
    "pošakis",
    "potyris",
    "potraukis",
    "potvarkis",
    "potvynis",
    "poveikis",
    "požemis",
    "požiūris",
    "požymis",
    "praeivis",
    "pramuštgalvis",
    "prieaugis",
    "priedėlis",
    "prieglobstis",
    "priekis",
    "priemiestis",
    "priemolis",
    "priepuolis",
    "prieskonis",
    "priesmėlis",
    "priešaušris",
    "prodiuseris",
    "proliuvis",
    "protarpis",
    "protrūkis",
    "proveržis",
    "pskovietis",
    "puodelis",
    "pusautomatis",
    "pusbrolis",
    "pusfabrikatis",
    "pusiasalis",
    "puslapis",
    "pusmetris",
    "pusrytis",
    "pustonis",
    "pusvalandis",
    "pušynėlis",
    "ragelis",
    "raištis",
    "raitelis",
    "raktelis",
    "ramstis",
    "rankinis",
    "rankogalis",
    "rankraštis",
    "ratelis",
    "rėmelis",
    "reporteris",
    "ridikėlis",
    "riteris",
    "rodiklis",
    "rožinis",
    "rublis",
    "runkelis",
    "rūpestis",
    "saldainis",
    "salezietis",
    "salietis",
    "sambūris",
    "sandėlis",
    "santykis",
    "sargybinis",
    "saugiklis",
    "saulėlydis",
    "saulėtekis",
    "sausainis",
    "savaitgalis",
    "savanoris",
    "savivardis",
    "savižudis",
    "sąjūdis",
    "sąlytis",
    "sąryšis",
    "sąskrydis",
    "seimelis",
    "sekmadienis",
    "senelis",
    "senis",
    "sepsis",
    "serveris",
    "siekis",
    "simbolis",
    "sykis",
    "skaitiklis",
    "skalpelis",
    "skambutis",
    "skardis",
    "skėtis",
    "skirsnis",
    "skydelis",
    "skyrelis",
    "skystis",
    "skliaustelis",
    "skonis",
    "skrandis",
    "skrydis",
    "skudutis",
    "slankstelis",
    "slapyvardis",
    "slėgis",
    "slėnis",
    "slenkstis",
    "sluoksnis",
    "smėlis",
    "smūgis",
    "snapelis",
    "snukis",
    "sodietis",
    "sovietmetis",
    "spalis",
    "sparnelis",
    "spektaklis",
    "spenelis",
    "spindis",
    "spygliuotis",
    "sprindis",
    "sraigtasparnis",
    "stabdis",
    "stačiakampis",
    "starkis",
    "stiebelis",
    "stiklainis",
    "stogelis",
    "straipsnis",
    "strypelis",
    "stulpelis",
    "sukilėlis",
    "sūnelis",
    "sunkvežimis",
    "suolelis",
    "suomis",
    "sūris",
    "sutuoktinis",
    "suvalkietis",
    "sužadėtinis",
    "svetimšalis",
    "svetimtautis",
    "svoris",
    "šakniavaisis",
    "šaligatvis",
    "šaltinis",
    "šaltis",
    "šaltkalvis",
    "šaukštelis",
    "šepetėlis",
    "šermukšnis",
    "šešėlis",
    "šeštadienis",
    "šiaulietis",
    "šiaurietis",
    "šikšnosparnis",
    "šimtmetis",
    "šiokiadienis",
    "šokis",
    "špilis",
    "špindelis",
    "šūkis",
    "šūksnis",
    "šunelis",
    "šuolis",
    "šūvis",
    "šventraštis",
    "šviesuolis",
    "tabelis",
    "takelis",
    "tamponėlis",
    "tamsuolis",
    "tarpelis",
    "tarpsnis",
    "tarpukaris",
    "tašelis",
    "taškelis",
    "tautietis",
    "telferis",
    "tėtis",
    "tėvelis",
    "tėvynainis",
    "tėvulis",
    "tinklelis",
    "titnagžemis",
    "tolis",
    "traleris",
    "transporteris",
    "trauklapis",
    "trečdalis",
    "trečiadienis",
    "treneris",
    "trikampis",
    "tritaškis",
    "triušis",
    "truputis",
    "tunelis",
    "turtuolis",
    "tvarkaraštis",
    "ūgis",
    "ūkis",
    "ukrainietis",
    "uošvis",
    "upelis",
    "upėtakis",
    "upokšnis",
    "uždarbis",
    "užjūris",
    "užkulisis",
    "užmokestis",
    "užsienietis",
    "užsienis",
    "vabalėlis",
    "vadovėlis",
    "vaikelis",
    "vaikigalis",
    "vaikis",
    "vaikutis",
    "vaismedis",
    "valgis",
    "valstietis",
    "vamzdelis",
    "vamzdis",
    "variklis",
    "varis",
    "varžtelis",
    "važis",
    "važtaraštis",
    "vedamasis",
    "veidelis",
    "veidrodis",
    "veiksmažodis",
    "velionis",
    "veršelis",
    "vežimėlis",
    "viduramžis",
    "vidurdienis",
    "vidurkis",
    "vienis",
    "vienkiemis",
    "vienuolis",
    "viešbutis",
    "vieškelis",
    "vietinis",
    "vietovardis",
    "vijoklis",
    "vilkijietis",
    "vilnietis",
    "vingis",
    "viršelis",
    "viršugalvis",
    "vyris",
    "vyriškis",
    "vytis",
    "vokietis",
    "volelis",
    "voratinklis",
    "žalgirietis",
    "žemaitis",
    "žemėlapis",
    "ženklelis",
    "žibintuvėlis",
    "žiburėlis",
    "žiedelis",
    "žiemgalis",
    "žingsnelis",
    "žingsnis",
    "žirnis",
    "žiupsnis",
    "žydkelis",
    "žygis",
    "žodelis",
    "žodis",
    "žvėrelis",
    "žvilgsnis",
    "žvirblis",
    "žvyrkelis",
];

static IS_THIRD_MASC: [&str; 1] = ["akis"];

static IS_THIRD_FEM: [&str; 1] = ["vagis"];
//}}}

pub fn decline_noun<'a>(
    word: String,
) -> Result<(String, Vec<(&'a str, [WordParts<'a>; 2])>), String> {
    for (ending, _declension_name, declensions) in DECLENSION_PATTERNS {
        if word.ends_with(ending) {
            let stem = word.strip_suffix(ending).unwrap_or(&word).to_owned();
            return Ok(parse_declensions(stem, declensions));
        }
    }
    if word.ends_with("is") {
        let stem = word.strip_suffix("is").unwrap_or(&word).to_owned();
        if IS_FIRST_DECL.contains(&word.as_str()) {
            return Ok(parse_declensions(stem, IS_1.2));
        } else if IS_THIRD_MASC.contains(&word.as_str()) {
            return Ok(parse_declensions(stem, IS_3M.2));
        } else if IS_THIRD_FEM.contains(&word.as_str()) {
            return Ok(parse_declensions(stem, IS_3F.2));
        }
    }
    Err("does not exit".to_owned())
}

fn parse_declensions<'a>(
    mut stem: String,
    declension: [[&'a str; 3]; 8],
) -> (String, Vec<(&'a str, [WordParts<'a>; 2])>) {
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

fn create_nonexistent_prefixes<'a>(declension: [&'a str; 3]) -> (&'a str, [WordParts<'a>; 2]) {
    (
        declension[0],
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
    )
}

fn handle_substitutions<'a>(
    original: &'a str,
    new: &'a str,
    declension: [[&'a str; 3]; 8],
) -> Vec<(&'a str, [WordParts<'a>; 2])> {
    let mut values = Vec::new();

    for [name, sing, plur] in declension {
        if let Some(plur_prefixed) = plur.strip_prefix("en") {
            if let Some(sing_prefixed) = sing.strip_prefix("en") {
                values.push((
                    name,
                    [
                        WordParts {
                            unmodified_stem: original,
                            modified_stem: "en",
                            ending: sing_prefixed,
                        },
                        WordParts {
                            unmodified_stem: original,
                            modified_stem: "en",
                            ending: plur_prefixed,
                        },
                    ],
                ))
            } else {
                values.push((
                    name,
                    [
                        WordParts {
                            unmodified_stem: original,
                            modified_stem: "",
                            ending: sing,
                        },
                        WordParts {
                            unmodified_stem: original,
                            modified_stem: "en",
                            ending: plur_prefixed,
                        },
                    ],
                ))
            }
        } else {
            let (sing_orig, sing_prefix, sing) = create_ending(sing, original, new);
            let (plur_orig, plur_prefix, plur) = create_ending(plur, original, new);
            values.push((
                name,
                [
                    WordParts {
                        unmodified_stem: sing_orig,
                        modified_stem: sing_prefix,
                        ending: sing,
                    },
                    WordParts {
                        unmodified_stem: plur_orig,
                        modified_stem: plur_prefix,
                        ending: plur,
                    },
                ],
            ))
        }
    }
    values
}
