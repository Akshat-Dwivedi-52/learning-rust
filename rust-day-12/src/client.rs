use tokio::{
    sync::{broadcast, mpsc},
    time::{interval, sleep, Duration},
};

use crate::{
    client_log,
    message::ChatMessage,
};

pub async fn run_client(
    name: String,
    tx: mpsc::Sender<ChatMessage>,
    mut rx: broadcast::Receiver<ChatMessage>,
) {
    let mut heartbeat = interval(Duration::from_secs(3));

    let timeout = sleep(Duration::from_secs(15));

    // Required because Sleep is not Unpin.
    tokio::pin!(timeout);

    let mut count = 1;

    loop {
        tokio::select! {

            // Receive broadcast messages
            Ok(message) = rx.recv() => {

                if message.sender != name {
                    client_log!(
                        &name,
                        "{} says: {}",
                        message.sender,
                        message.content
                    );
                }

            }

            // Send heartbeat + a demo chat message
            _ = heartbeat.tick() => {

                client_log!(&name, "❤️ Heartbeat");

                let message = ChatMessage::new(
                    &name,
                    &format!("Hello {}", count),
                );

                if tx.send(message).await.is_err() {
                    break;
                }

                count += 1;
            }

            // Client timeout
            _ = &mut timeout => {

                client_log!(&name, "⏰ Timed Out.");

                break;
            }
        }
    }

    client_log!(&name, "Disconnected.");
}