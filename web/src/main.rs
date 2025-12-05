use api::server_functions::{create_todo, delete_todo, get_all_todos, update_todo};
use dioxus::{
    logger::{self, tracing},
    prelude::*,
};
use shared::models;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

static TODOS: GlobalSignal<Vec<models::Todo>> = GlobalSignal::new(Vec::new);

fn main() {
    logger::init(tracing::Level::INFO).expect("failed to init logger");
    tracing::info!("starting app");
    launch(App);
}

#[derive(Clone, Routable)]
enum Route {
    #[route("/")]
    Home {},
}

#[component]
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

#[component]
pub fn AddTodo() -> Element {
    let mut title = use_signal(String::new);

    rsx! {
        div { class: "mb-4",
            input {
                class: "border p-2 rounded-sm mr-2",
                placeholder: "Title",
                value: "{title}",
                oninput: move |event| title.set(event.value()),
            }
            button {
                class: "border p-1 rounded-sm",
                onclick: move |_| async move {
                    match create_todo(title()).await {
                        Ok(id) => TODOS.write().push(models::Todo::new(id, title())),
                        Err(err) => tracing::error!("create todo error: {err}"),
                    }
                },
                "Add Todo"
            }
        }
    }
}

#[component]
pub fn Todos() -> Element {
    let resource = use_resource(|| async {
        match get_all_todos().await {
            Ok(todos) => TODOS.write().extend(todos),
            Err(err) => tracing::error!("get all todos error: {err}"),
        }
    });

    match resource() {
        Some(_) if !TODOS().is_empty() => {
            rsx! {
                for todo in TODOS() {
                    Todo { key: "{todo.id}", todo }
                }
            }
        }
        _ => rsx! { "No Todos Yet" },
    }
}

#[component]
fn Todo(todo: models::Todo) -> Element {
    let mut title = use_signal(|| todo.title);
    let mut completed = use_signal(|| todo.completed);

    rsx! {
        div { class: "my-1",
            input {
                name: "completed",
                r#type: "checkbox",
                checked: "{completed}",
                oninput: move |event| {
                    if let Ok(as_bool) = event.value().parse() {
                        completed.set(as_bool);
                    }
                },
            }
            input {
                class: "border p-2 rounded-sm mx-2",
                name: "title",
                value: "{title}",
                oninput: move |event| title.set(event.value()),
            }
            button {
                class: "border p-1 rounded-sm",
                class: "mx-3",
                onclick: move |_| async move {
                    match update_todo(todo.id, title(), completed()).await {
                        Ok(_) => {
                            if let Some(todo) = TODOS.write().iter_mut().find(|t| t.id == todo.id) {
                                todo.title = title();
                                todo.completed = completed();
                            }
                        }
                        Err(err) => tracing::error!("update todo error: {err}"),
                    }
                },
                "update"
            }
            button {
                class: "border p-1 rounded-sm",
                onclick: move |_| async move {
                    match delete_todo(todo.id).await {
                        Ok(_) => TODOS.write().retain(|t| t.id != todo.id),
                        Err(err) => tracing::error!("delete todo error: {err}"),
                    }
                },
                "delete"
            }
        }
    }
}
