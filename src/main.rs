use leptos::{ev::SubmitEvent, html::Input, *};
use leptos_router::*;

mod declension;
mod game;

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
              <Route path="/streak" view=game::GameView/>
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
            .map(|[declension, sing_pref, plur_pref, sing, plur]| {
                view! { cx,
                    <tr class={if *declension == "Illative" {"Muted"} else {"Normal"}} >
                        <td class={declension}>{declension}</td>
                        <td>{&stem}<div class="changed-stem" style="display: inline">{sing_pref}</div><div class={declension} style="display: inline">{sing}</div></td>
                        <td>{&stem}<div class="changed-stem" style="display: inline">{plur_pref}</div><div class={declension} style="display: inline">{plur}</div></td>
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
