use dioxus::prelude::*;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

mod message_worker;
mod openai_client;

#[cfg(feature = "server")]
#[tokio::main]
async fn main() {
    dioxus::logger::initialize_default();
    let address = dioxus::cli_config::fullstack_address_or_localhost();

    let worker_handle = message_worker::start_producer_worker();

    let router = axum::Router::new().serve_dioxus_application(ServeConfigBuilder::default(), App);

    let router = router.into_make_service();
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    axum::serve(listener, router).await.unwrap();
    worker_handle.abort();
}

#[cfg(not(feature = "server"))]
fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS } document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Chat {}
    }
}

#[component]
pub fn Chat() -> Element {
    rsx! {
        div {
            ChatMessage {
                username: "Test".to_string(),
                message: "Test".to_string()
            }
        }
    }
}

#[component]
pub fn ChatMessage(username: String, message: String) -> Element {
    rsx! {
        div {
            class: "",
            p {
                span {
                    class: "text-yellow-500",
                    "{username}"
                }
                ": {message}"
            }
        }
    }
}
