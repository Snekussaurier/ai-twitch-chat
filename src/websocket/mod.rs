mod handler;

use axum::routing::get;
use axum::Router;
use dioxus::logger::tracing::debug;
use std::net::SocketAddr;

pub async fn run_websocket_server() {
    let app = Router::new().route("/ws", get(handler::ws_handler));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3030));
    debug!("WebSocket server running on ws://{}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}
