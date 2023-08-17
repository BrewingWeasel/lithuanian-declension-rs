use crate::declension;
use leptos::{ev::SubmitEvent, html::Input, *};
use rand::seq::SliceRandom;
use rand::Rng;

// Generated with awk `'$2 == "N" {print $1}' JCL_lemmas.txt | head -n 300 | sed -E 's/(.*)/"\1",/' > based on JCL wordlist`
static POSSIBLE_WORDS: [&str; 300] = [
    "metai",
    "straipsnis",
    "įstatymas",
    "respublika",
    "valstybė",
    "darbas",
    "projektas",
    "dalis",
    "teisė",
    "žmogus",
    "diena",
    "seimas",
    "asmuo",
    "vyriausybė",
    "šalis",
    "vieta",
    "savivaldybė",
    "įmonė",
    "metas",
    "narys",
    "litas",
    "teismas",
    "vaikas",
    "veikla",
    "tvarka",
    "žemė",
    "ministerija",
    "pirmininkas",
    "laikas",
    "miestas",
    "sprendimas",
    "pakeitimas",
    "komisija",
    "priemonė",
    "punktas",
    "apsauga",
    "tarnyba",
    "ministras",
    "kartas",
    "įstaiga",
    "taryba",
    "ūkis",
    "nutarimas",
    "klausimas",
    "turtas",
    "sutartis",
    "bendrovė",
    "dokumentas",
    "institucija",
    "mokestis",
    "programa",
    "vadovas",
    "rajonas",
    "komitetas",
    "pasaulis",
    "duomuo",
    "sveikata",
    "pareigūnas",
    "mėnuo",
    "atvejis",
    "grupė",
    "tyrimas",
    "moteris",
    "paslauga",
    "informacija",
    "automobilis",
    "gyventojas",
    "sistema",
    "mokykla",
    "aktas",
    "vyras",
    "policija",
    "centras",
    "prezidentas",
    "rinkimai",
    "kaina",
    "reikalavimas",
    "sąjunga",
    "lietuvis",
    "gyvenimas",
    "skyrius",
    "bankas",
    "žodis",
    "namai",
    "užsienis",
    "lėšos",
    "darbuotojas",
    "pinigas",
    "draudimas",
    "reikalas",
    "byla",
    "komanda",
    "atstovas",
    "galimybė",
    "departamentas",
    "sąlyga",
    "biudžetas",
    "kultūra",
    "vanduo",
    "partija",
    "visuomenė",
    "taisyklė",
    "šeima",
    "direktorius",
    "fondas",
    "teritorija",
    "parama",
    "kelias",
    "pasiūlymas",
    "aplinka",
    "pažeidimas",
    "posėdis",
    "organizacija",
    "mokslas",
    "rinka",
    "kalba",
    "rezultatas",
    "nuostata",
    "būdas",
    "nuomonė",
    "savaitė",
    "pajamos",
    "verslas",
    "medžiaga",
    "problema",
    "skaičius",
    "prekyba",
    "priežiūra",
    "pradžia",
    "prekė",
    "vidus",
    "transportas",
    "produktas",
    "tikslas",
    "gatvė",
    "klubas",
    "akcija",
    "pusė",
    "pastatas",
    "planas",
    "sritis",
    "kaimas",
    "sąrašas",
    "ryšys",
    "prašymas",
    "bauda",
    "pensija",
    "veiksmas",
    "valdžia",
    "suma",
    "finansai",
    "kontrolė",
    "išlaidos",
    "pagalba",
    "pilietis",
    "vardas",
    "oras",
    "statyba",
    "valdymas",
    "kodeksas",
    "sportas",
    "taškas",
    "leidimas",
    "dėmesys",
    "pavadinimas",
    "saugumas",
    "maistas",
    "rungtynės",
    "liga",
    "specialistas",
    "laikotarpis",
    "pabaiga",
    "dydis",
    "politika",
    "procentas",
    "sausis",
    "papildymas",
    "dalyvis",
    "santykis",
    "licencija",
    "pavyzdys",
    "dalykas",
    "laisvė",
    "terminas",
    "rytas",
    "objektas",
    "komentaras",
    "konkursas",
    "išvada",
    "balsas",
    "vertė",
    "istorija",
    "gairė",
    "žinios",
    "švietimas",
    "situacija",
    "administracija",
    "naujiena",
    "tiesa",
    "čempionatas",
    "įgyvendinimas",
    "procesas",
    "ranka",
    "universitetas",
    "karas",
    "pranešimas",
    "valanda",
    "priedas",
    "butas",
    "patalpa",
    "liepa",
    "kolega",
    "gruodis",
    "plėtra",
    "energija",
    "daugelis",
    "laivas",
    "forma",
    "įsakymas",
    "ekonomika",
    "rinktinė",
    "miškas",
    "knyga",
    "tinklas",
    "registras",
    "pareigos",
    "naudojimas",
    "teisėjas",
    "muzika",
    "pataisa",
    "savininkas",
    "amžius",
    "filmas",
    "dujos",
    "narė",
    "plotas",
    "gamyba",
    "valdyba",
    "renginys",
    "pergalė",
    "televizija",
    "galva",
    "priežastis",
    "gegužė",
    "kraštas",
    "data",
    "pagrindas",
    "vakaras",
    "birželis",
    "kodas",
    "apskritis",
    "įvykis",
    "elektra",
    "patvirtinimas",
    "vairuotojas",
    "jūra",
    "namas",
    "kovas",
    "krepšinis",
    "vertinimas",
    "kova",
    "nusikaltimas",
    "balandis",
    "funkcija",
    "spauda",
    "atsakomybė",
    "sezonas",
    "lygis",
    "pavaduotojas",
    "teatras",
    "galia",
    "dauguma",
    "balsavimas",
    "rugsėjis",
    "atlyginimas",
    "šventė",
    "priėmimas",
    "tauta",
    "internetas",
    "sklypas",
    "mokėjimas",
    "ketvirtadienis",
    "spalis",
    "pirkimas",
    "tėvai",
    "lapkritis",
    "subjektas",
    "ligoninė",
    "vartotojas",
    "agentūra",
    "meras",
    "draugas",
    "kinas",
    "lyderis",
    "ataskaita",
    "euras",
    "nuosavybė",
    "pastaba",
    "telefonas",
    "raštas",
];

static CASES: [&str; 7] = [
    "Nominative",
    "Genitive",
    "Dative",
    "Accusative",
    "Instrumental",
    "Locative",
    "Vocative",
];

static NUMBERS: [&str; 2] = ["Singular", "Plural"];

#[component]
pub fn GameView(cx: Scope) -> impl IntoView {
    let (streak, set_streak) = create_signal(cx, 0);
    let (answer, set_answer) = create_signal(cx, String::new());

    let (word, set_word) = create_signal(cx, "");
    let (number, set_number) = create_signal(cx, 0);
    let (case, set_case) = create_signal(cx, 0);
    let input_element: NodeRef<Input> = create_node_ref(cx);

    let create_word = move || {
        let word = POSSIBLE_WORDS.choose(&mut rand::thread_rng()).unwrap();
        set_word(word);
        set_number(rand::thread_rng().gen_range(0..=1));
        set_case(rand::thread_rng().gen_range(0..7));
    };

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();

        let value = input_element().expect("<input> to exist").value();
        let (stem, endings) = declension::decline(word().to_owned()).unwrap();
        let real_word = stem + &endings[case()][number() + 1];
        if value == real_word {
            set_streak.update(|x| *x += 1);
            set_answer(String::from("correct!"));
        } else {
            set_streak(0);
            let response = format!("incorrect! Should be {}", real_word);
            set_answer(response);
        }
        create_word();
        input_element().unwrap().set_value("");
    };
    create_word();
    view! { cx,
        <h1>Current streak: {streak}</h1>
        <h2>Find the {move || NUMBERS[number()]}{" "}{move || CASES[case()]} of {word}</h2>
        <form on:submit=on_submit class="word_input">
            <input type="text"
                node_ref=input_element
            />
            <input type="submit" class="button" value="Guess"/>
        </form>
        <h1>{answer}</h1>
    }
}
