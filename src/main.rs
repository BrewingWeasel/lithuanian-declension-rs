#![feature(let_chains)]
use leptos::{ev::SubmitEvent, html::Input, *};
use leptos_router::*;
use noundecl::decline;

mod game;

fn main() {
    mount_to_body(|cx| view! { cx,  <App/> })
}

#[derive(Params, PartialEq)]
struct WordSearch {
    word: String,
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
              <Route path="/decline" view=DeclensionPage/>
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
        <form action="decline/" class="word_input">
            <input type="text"
                value=word
                class="search_input"
                name="word"
                node_ref=input_element
            />
            <input type="submit" class="button" value="Generate"/>
        </form>
        <div class="contents"
             style:display=move || if word().is_empty() { "none" } else { "block" }
            >
            <h1>"Declension pattern for " {word}</h1>
            // <DeclinedWords info=word />
        </div>
    }
}

#[component]
fn DeclensionPage(cx: Scope) -> impl IntoView {
    let query = use_query::<WordSearch>(cx);
    let word = query.with(|v| {
        v.as_ref()
            .map(|query| query.word.clone())
            .unwrap_or(String::new())
    });
    view! {cx,
        <div class="contents"
             style:display=if word.is_empty() { "none" } else { "block" }
            >
            <h1>"Declension pattern for " {&word}</h1>
            <DeclinedWords info=word.to_string()/>
        </div>
    }
}

#[component]
fn DeclinedWords(cx: Scope, info: String) -> impl IntoView {
    let declension = match decline(info.clone()) {
        Ok(noundecl::Declension::Noun(stem, decl)) => {
            let declensions = decl.iter()
                    .map(|(declension, [sing, plur])| {
                        view! { cx,
                            <tr class={if *declension == "Illative" {"Muted"} else {"Normal"}} >
                                <td class={*declension}>{*declension}</td>
                                <td>{&stem}{sing.unmodified_stem}<div class="changed-stem" style="display: inline">{sing.modified_stem}</div><div class={*declension} style="display: inline">{sing.ending}</div></td>
                                <td>{&stem}{plur.unmodified_stem}<div class="changed-stem" style="display: inline">{plur.modified_stem}</div><div class={*declension} style="display: inline">{plur.ending}</div></td>
                            </tr>
                        }
                    })
                    .collect_view(cx);

            view! { cx,
                <table>
                    <tr>
                        <th>Declension</th>
                        <th>Singular</th>
                        <th>Plural</th>
                    </tr>
                    {declensions}
                </table>
            }
            .into_view(cx)
        }

        Ok(noundecl::Declension::Adjective(stem, decl)) => {
            let view = decl.iter()
                .map(|(declension, [[mascsg, femsg], [mascpl, fempl]])|
                    (view! { cx,
                        <tr class={if *declension == "Illative" {"Muted"} else {"Normal"}} >
                            <td class={*declension}>{*declension}</td>
                            <td>{&stem}{mascsg.unmodified_stem}<div class="changed-stem" style="display: inline">{mascsg.modified_stem}</div><div class={*declension} style="display: inline">{mascsg.ending}</div></td>
                            <td>{&stem}{mascpl.unmodified_stem}<div class="changed-stem" style="display: inline">{mascpl.modified_stem}</div><div class={*declension} style="display: inline">{mascpl.ending}</div></td>
                        </tr>
                    },
                    view! { cx,
                        <tr class={if *declension == "Illative" {"Muted"} else {"Normal"}} >
                            <td class={*declension}>{*declension}</td>
                            <td>{&stem}{femsg.unmodified_stem}<div class="changed-stem" style="display: inline">{femsg.modified_stem}</div><div class={*declension} style="display: inline">{femsg.ending}</div></td>
                            <td>{&stem}{fempl.unmodified_stem}<div class="changed-stem" style="display: inline">{fempl.modified_stem}</div><div class={*declension} style="display: inline">{fempl.ending}</div></td>
                        </tr>
                    }
                    )
                );

            // TODO: don't use extra clone
            view! { cx,
                <table class="neuter-value">
                    <tr>
                        <th>Neuter</th>
                    </tr>
                    <tr>
                        <td>
                            {info[..info.len() - 1].to_string()}
                        </td>
                    </tr>
                </table>
                <div class="titles">
                    <h2>Masculine</h2>
                    <h2>Feminine</h2>
                </div>
                <div class="tables">
                <table>
                    <tr>
                        <th>Declension</th>
                        <th>Singular</th>
                        <th>Plural</th>
                    </tr>
                    {view.clone().map(|(m, _)| m).collect_view(cx)}
                </table>
                <table>
                    <tr>
                        <th>Declension</th>
                        <th>Singular</th>
                        <th>Plural</th>
                    </tr>
                    {view.map(|(_, f)| f).collect_view(cx)}
                </table></div>
            }
            .into_view(cx)
        }
        Err(_) => View::default(),
    };
    declension
}
