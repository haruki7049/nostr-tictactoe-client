use dioxus::prelude::*;
use tictactoe::components;

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
            Router::<components::Route> {}
        }
    }
}
