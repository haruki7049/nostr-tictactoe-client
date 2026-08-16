use dioxus::prelude::*;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Game {},
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        body {
            class: "scheme-light-dark font-sans bg-stone-100 text-dark my-0 mx-auto pt-8 px-5 max-w-5xl",

            document::Link { rel: "icon", href: FAVICON }
            document::Link { rel: "stylesheet", href: TAILWIND_CSS }
            Router::<Route> {}
        }
    }
}

#[component]
fn Game() -> Element {
    rsx! {
        div {
            class: "grid gap-4 items-start grid-cols-2",
            Board { }
            Panels { }
        }
    }
}

/// Board
#[component]
fn Board() -> Element {
    rsx! {
        div {
            class: "block border border-line rounded-card bg-card",
            p { "This is an example text..." }
            p { "This is an example text..." }
        }
    }
}

/// Panels
#[component]
fn Panels() -> Element {
    rsx! {
        div {
            class: "border border-line rounded-card bg-card",
            p { "This is an example text..." }
            p { "This is an example text..." }
        }
    }
}

/// Shared navbar component.
#[component]
fn Navbar() -> Element {
    rsx! {
        header {
            class: "flex gap-4 mb-5 text-xl font-bold justify-between items-center",
            id: "navbar",
            Link {
                to: Route::Game {},
                "Tictactoe"
            }

            NostrLogin { }
        }

        Outlet::<Route> {}
    }
}

#[component]
fn NostrLogin() -> Element {
    rsx! {
        button {
            class: "bg-slate-200 px-2 py-3 rounded-full",
            "Login with Nostr: TODO"
        }
    }
}
