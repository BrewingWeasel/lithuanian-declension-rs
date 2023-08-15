use leptos::{ev::SubmitEvent, html::Input, *};

fn main() {
    mount_to_body(|cx| view! { cx,  <App/> })
}

#[component]
fn App(cx: Scope) -> impl IntoView {
    view! { cx,
        <TextInp/>
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

fn decline<'a>(word: String) -> (String, Vec<(&'a str, &'a str, &'a str)>) {
    let declension_patterns = [
        ("Nominative", "as", "ai"),
        ("Genitive", "o", "ų"),
        ("Dative", "ui", "ams"),
        ("Accusative", "ą", "us"),
        ("Instrumental", "u", "ais"),
        ("Locative", "e", "uose"),
        ("Vocative", "ai", "ai"),
    ];
    let stem = word.strip_suffix("as").unwrap_or(&word).to_owned();
    let mut declension = Vec::new();
    for (cur_declension, ending_sing, ending_plur) in declension_patterns {
        declension.push((cur_declension, ending_sing, ending_plur));
    }
    (stem, declension)
}

#[component]
fn DeclinedWords(cx: Scope, info: ReadSignal<String>) -> impl IntoView {
    let counter_buttons = move || {
        let (stem, decl) = decline(info());
        decl.iter()
            .map(|(declension, sing, plur)| {
                view! { cx,
                    <tr>
                        <td>{*declension}</td>
                        <td>{&stem}{*sing}</td>
                        <td>{&stem}{*plur}</td>
                    </tr>
                }
            })
            .collect_view(cx)
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
