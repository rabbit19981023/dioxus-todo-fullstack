#![allow(non_snake_case)]

#[cfg(feature = "server")]
mod database;

mod components;
mod models;
mod server_functions;

use dioxus::prelude::*;
use dioxus_logger::tracing;

use components::{AddTodo, Todos};

#[derive(Clone, Routable)]
enum Route {
    #[route("/")]
    Home {},
}

fn main() {
    // Init logger
    dioxus_logger::init(tracing::Level::INFO).expect("failed to init logger");
    tracing::info!("starting app");
    launch(App);
}

fn App() -> Element {
    rsx! {
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
