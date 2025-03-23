use base64::prelude::BASE64_STANDARD;
use base64::{engine::general_purpose, Engine as _};
use dioxus::logger::tracing::error;
use fs_extra::dir;
use std::io::Cursor;
use std::time::Duration;
use tokio::task;
use tokio::time::sleep;
use xcap::Monitor;

use crate::message_worker::buffer::get_sender;
use crate::message_worker::types::deserialize_data;
use crate::openai_client::call_openai_api;

pub fn start_producer_worker() -> tokio::task::JoinHandle<()> {
    task::spawn(async move {
        let tx = get_sender().await;
        let monitors = Monitor::all().unwrap();
        dir::create_all("target/monitors", true).unwrap();

        loop {
            // Take a screenshot of the display
            let mut screenshot_b64 = String::new();
            for monitor in &monitors {
                if monitor.is_primary().unwrap() {
                    let image = monitor.capture_image().unwrap();
                    let mut bytes: Vec<u8> = Vec::new();
                    let _ = image.write_to(&mut Cursor::new(&mut bytes), image::ImageFormat::Png);
                    screenshot_b64 = BASE64_STANDARD.encode(bytes);
                }
            }

            let response = call_openai_api(
                "Test Input. Chat Ich bin am testen. Nicht ausrasten".to_string(),
                screenshot_b64,
            )
            .await;
            match response {
                Ok(success) => {
                    let deserialized_data = deserialize_data(success);
                    if let Ok(messages) = deserialized_data {
                        for message in messages {
                            if tx.send(message).await.is_err() {
                                println!("Receiver dropped, stopping producer.");
                                break;
                            }
                        }
                    }
                }
                Err(err) => error!("{}", err),
            }
            sleep(Duration::from_secs(20)).await;
        }
    })
}
