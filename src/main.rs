use dioxus::logger::tracing::{debug, error};
use dioxus::prelude::*;
use futures_util::stream::StreamExt;
use message_worker::producer::start_producer_worker;
use message_worker::types::Message;
use std::time::Duration;
use tokio::time::sleep;
use tokio_tungstenite::connect_async;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

mod message_worker;
mod openai_client;
mod websocket;

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let _websocket = spawn(async {
        let _ = tokio::spawn(async {
            websocket::run_websocket_server().await;
        })
        .await;
    });
    let _producer_worker = spawn(async {
        let _ = start_producer_worker().await;
    });
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS } document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Chat {}
    }
}

#[component]
pub fn Chat() -> Element {
    let mut messages = use_signal(Vec::<message_worker::types::Message>::new);

    let _tx: Coroutine<String> = use_coroutine(move |_rx: UnboundedReceiver<String>| async move {
        let url = "ws://127.0.0.1:3030/ws";
        debug!("Connecting to websocket at {url}");

        debug!("Connected to websicket.");

        let ws_stream = loop {
            debug!("Trying to connect to WebSocket at {url}...");
            match connect_async(url).await {
                Ok((ws_stream, _)) => {
                    println!("WebSocket handshake successful!");
                    break ws_stream;
                }
                Err(err) => {
                    error!("Failed to connect: {}. Retrying in 1 second...", err);
                    sleep(Duration::from_secs(1)).await;
                }
            }
        };
        println!("WebSocket handshake has been successfully completed");

        let (_, mut read) = ws_stream.split();
        while let Some(msg) = read.next().await {
            match msg {
                Ok(msg) => messages
                    .write()
                    .push(serde_json::from_str(&msg.to_string()).unwrap()),
                Err(err) => error!("Fuck! {}", err),
            }
        }
        debug!("Disconnected from websicket");
    });
    rsx! {
        for message in messages() {
            ChatMessage { message }
        }
    }
}

const USER_COLOR_CLASSES: &[&str] = &[
    "text-red-500",
    "text-orange-500",
    "text-yellow-500",
    "text-green-500",
    "text-teal-500",
    "text-blue-500",
    "text-indigo-500",
    "text-purple-500",
    "text-pink-500",
    "text-lime-500",
    "text-fuchsia-500",
];

#[component]
pub fn ChatMessage(message: Message) -> Element {
    let color_class = {
        let sum: usize = message.username.bytes().map(|b| b as usize).sum();
        let index = sum % USER_COLOR_CLASSES.len();
        USER_COLOR_CLASSES[index]
    };

    rsx! {
        div {
            class: "",
            p {
                span {
                    class: "{color_class} text-orange-500",
                    "{message.username}"
                }
                ": {message.message}"
            }
        }
    }
}
