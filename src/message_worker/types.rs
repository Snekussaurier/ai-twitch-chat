use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};

pub type SenderMessageTuple = (mpsc::Sender<Message>, Arc<Mutex<mpsc::Receiver<Message>>>);

#[derive(Debug, Deserialize, Serialize, Props, Clone, PartialEq)]
pub struct Message {
    pub username: String,
    pub message: String,
}

pub fn deserialize_data(data: String) -> Result<Vec<Message>, serde_json::Error> {
    serde_json::from_str::<Vec<Message>>(&data)
}
