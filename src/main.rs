use leptos::{ev::SubmitEvent, html::Input, *};

fn main() {
    println!("Hello, world!");
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
    let (name, set_name) = create_signal(cx, "Labas".to_string());
    let input_element: NodeRef<Input> = create_node_ref(cx);
    let on_submit = move |ev: SubmitEvent| {
        // stop the page from reloading!
        ev.prevent_default();

        let value = input_element().expect("<input> to exist").value();
        set_name(value);
    };
    view! { cx,
        <form on:submit=on_submit>
            <input type="text"
                value=name
                node_ref=input_element
            />
            <input type="submit" value="Generate"/>
        </form>
        <p>"Declension pattern for " {name}</p>
        <DeclinedWords info=name />
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
    let stem = word.strip_suffix("as").unwrap().to_owned();
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
