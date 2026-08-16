use dioxus::prelude::*;

use crate::types;

/// Router
#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Toppage {},
}

/// Toppage
#[component]
fn Toppage() -> Element {
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
    let mut board = use_signal(types::Board::default);

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
                to: Route::Toppage {},
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
