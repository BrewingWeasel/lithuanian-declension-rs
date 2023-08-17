use leptos::{ev::SubmitEvent, html::Input, *};
use leptos_router::*;
use rand::seq::SliceRandom;
use rand::Rng;

mod declension;

fn main() {
    mount_to_body(|cx| view! { cx,  <App/> })
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    view! { cx,
        <Router>
        <nav>
        </nav>
        <main>
            <Routes>
              <Route path="/" view=TextInp/>
              <Route path="/streak" view=GameView/>
            </Routes>
        </main>
        </Router>
    }
}

#[component]
fn TextInp(cx: Scope) -> impl IntoView {
    let (word, set_word) = create_signal(cx, String::new());
    let input_element: NodeRef<Input> = create_node_ref(cx);
    let on_submit = move |ev: SubmitEvent| {
        // stop the page from reloading!
        ev.prevent_default();

        let value = input_element().expect("<input> to exist").value();
        set_word(value);
    };

    view! { cx,
        <form on:submit=on_submit class="word_input">
            <input type="text"
                value=word
                class="search_input"
                node_ref=input_element
            />
            <input type="submit" class="button" value="Generate"/>
        </form>
        <div class="contents"
             style:display=move || if word().is_empty() { "none" } else { "block" }
            >
            <h1>"Declension pattern for " {word}</h1>
            <DeclinedWords info=word />
        </div>
    }
}

#[component]
fn DeclinedWords(cx: Scope, info: ReadSignal<String>) -> impl IntoView {
    let counter_buttons = move || match declension::decline(info()) {
        Ok(val) => {
            let (stem, decl) = val;
            decl.iter()
            .map(|[declension, sing, plur]| {
                view! { cx,
                    <tr class={if *declension == "Illative" {"Muted"} else {"Normal"}} >
                        <td class={declension}>{declension}</td>
                        <td>{&stem}<div class={declension} style="display: inline">{sing}</div></td>
                        <td>{&stem}<div class={declension} style="display: inline">{plur}</div></td>
                    </tr>
                }
            })
            .collect_view(cx)
        }
        Err(_) => leptos::View::default(),
    };

    view! { cx,
        <table>
            <tr>
                <th>Declension</th>
                <th>Singular</th>
                <th>Plural</th>
            </tr>
            {counter_buttons}
        </table>
    }
}

static POSSIBLE_WORDS: [&str; 3] = ["žodis", "flamingas", "lūpas"];

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
fn GameView(cx: Scope) -> impl IntoView {
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
