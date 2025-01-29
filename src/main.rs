#![allow(non_snake_case)]

#[cfg(feature = "server")]
mod database;

mod components;
mod models;
mod server_functions;

use dioxus::{
    logger::{self, tracing},
    prelude::*,
};

use components::{AddTodo, Todos};

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

#[derive(Clone, Routable)]
enum Route {
    #[route("/")]
    Home {},
}

fn main() {
    // Init logger
    logger::init(tracing::Level::INFO).expect("failed to init logger");
    tracing::info!("starting app");
    launch(App);
}

fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }

        Router::<Route> {}
    }
}

#[component]
fn Home() -> Element {
    rsx! {
        div { class: "h-screen flex flex-col justify-center items-center",
            AddTodo {}
            Todos {}
        }
    }
}
