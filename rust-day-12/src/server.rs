use tokio::sync::{broadcast, mpsc};

use crate::{
    message::ChatMessage,
    server_log,
};

pub async fn run_server(mut incoming: mpsc::Receiver<ChatMessage>, broadcaster: broadcast::Sender<ChatMessage>) {
    server_log!("Server Started.");

    while let Some(message) = incoming.recv().await {
        server_log!("{}: {}", message.sender, message.content);
        let _ = broadcaster.send(message);
    }

    server_log!("Server Shutdown.");
}