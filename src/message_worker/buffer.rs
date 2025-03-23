use crate::message_worker::types::Message;
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::sync::OnceCell;

pub type MessageType = Message; // Define message type

static SENDER: OnceCell<Sender<MessageType>> = OnceCell::const_new();
static RECEIVER: OnceCell<tokio::sync::Mutex<Option<Receiver<MessageType>>>> =
    OnceCell::const_new();

// Initialize both cells if needed
async fn ensure_initialized() {
    if SENDER.get().is_none() {
        let (tx, rx) = mpsc::channel::<MessageType>(100);
        let _ = SENDER.set(tx);
        let _ = RECEIVER.set(tokio::sync::Mutex::new(Some(rx)));
    }
}

// Get a clone of the sender
pub async fn get_sender() -> Sender<MessageType> {
    ensure_initialized().await;
    SENDER.get().unwrap().clone()
}

pub async fn take_receiver() -> Option<Receiver<MessageType>> {
    ensure_initialized().await;

    let mutex = RECEIVER.get().unwrap();
    let mut guard = mutex.lock().await;
    guard.take()
}
