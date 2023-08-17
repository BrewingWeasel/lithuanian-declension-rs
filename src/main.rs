use leptos::{ev::SubmitEvent, html::Input, *};

mod declension;

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

#[component]
fn DeclinedWords(cx: Scope, info: ReadSignal<String>) -> impl IntoView {
    let counter_buttons = move || match declension::decline(info()) {
        Ok(val) => {
            let (stem, decl) = val;
            decl.iter()
            .map(|(declension, sing, plur)| {
                view! { cx,
                    <tr class={if *declension == "Illative" {"Muted"} else {"Normal"}} >
                        <td class={*declension}>{*declension}</td>
                        <td>{&stem}<div class={*declension} style="display: inline">{sing}</div></td>
                        <td>{&stem}<div class={*declension} style="display: inline">{plur}</div></td>
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
