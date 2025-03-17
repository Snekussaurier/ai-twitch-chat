use serde::Deserialize;

#[cfg(feature = "server")]
pub fn start_producer_worker() -> tokio::task::JoinHandle<()> {
    use dioxus::logger::tracing::debug;
    use fs_extra::dir;
    use std::time::Duration;
    use std::time::Instant;
    use tokio::sync::mpsc;
    use tokio::task;
    use tokio::time::sleep;
    use xcap::Monitor;

    use crate::openai_client::openai_client;

    task::spawn(async move {
        let monitors = Monitor::all().unwrap();

        for monitor in monitors {
            if monitor.is_primary() {
                let image = monitor
                    .capture_image()
                    .unwrap()
                    .save(format!(
                        "target/monitors/monitor-{}.png",
                        normalized(monitor.name().unwrap())
                    ))
                    .unwrap();
            }
        }
        loop {
            // Take a screenshot of the display

            let response = openai_client::call_openai_api(
                "Test Input. Chat Ich bin am testen. Nicht ausrasten".to_string(),
            )
            .await;
            match response {
                Ok(success) => {
                    let deserialized_data = deseriliaze_data(success);
                    if let Ok(success_data) = deserialized_data {}
                }
                Err(_) => debug!("Fuck"),
            }
            sleep(Duration::from_secs(15)).await;
        }
    })
}

fn deseriliaze_data(data: String) -> Result<Vec<Message>, serde_json::Error> {
    serde_json::from_str::<Vec<Message>>(&data)
}

fn normalized(filename: String) -> String {
    filename.replace(['|', '\\', ':', '/'], "")
}

#[derive(Debug, Deserialize)]
pub struct Message {
    username: String,
    message: String,
}
