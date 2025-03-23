use axum::extract::ws::CloseFrame;
use axum::extract::ws::{Message, Utf8Bytes, WebSocket, WebSocketUpgrade};
use axum::response::Response;
use dioxus::logger::tracing::{debug, error, info};
use futures_util::{sink::SinkExt, stream::StreamExt};

use crate::message_worker::buffer::take_receiver;

pub async fn ws_handler(ws: WebSocketUpgrade) -> Response {
    debug!("Got incoming websocket connection.");
    ws.on_upgrade(handle_socket)
}

async fn handle_socket(socket: WebSocket) {
    let (mut sender, mut reciever) = socket.split();
    let mut rx = take_receiver().await.unwrap();

    let send_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if sender
                .send(Message::Text(serde_json::to_string(&msg).unwrap().into()))
                .await
                .is_err()
            {
                error!("Error while sending web socket");
                break;
            }
            tokio::time::sleep(std::time::Duration::from_millis(1000)).await;
        }

        info!("Sending close...");
        if let Err(e) = sender
            .send(Message::Close(Some(CloseFrame {
                code: axum::extract::ws::close_code::NORMAL,
                reason: Utf8Bytes::from_static("Goodbye"),
            })))
            .await
        {
            error!("Could not send Close due to {e}, probably it is ok?");
        }
    });

    while let Some(Ok(msg)) = reciever.next().await {
        let _ = match msg {
            Message::Text(_) => None,
            Message::Close(..) => {
                send_task.abort();
                Some(())
            }
            Message::Binary(_) => None,
            Message::Ping(_) => None,
            Message::Pong(_) => None,
        };
    }

    // returning from the handler closes the websocket connection
    info!("Websocket context destroyed");
}
