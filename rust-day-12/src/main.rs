mod client;
mod macros;
mod message;
mod server;

use tokio::sync::{broadcast, mpsc};

use message::ChatMessage;

#[tokio::main]
async fn main() {
    // Channel: Clients -> Server
    let (server_tx, server_rx) = mpsc::channel::<ChatMessage>(100);

    // Channel: Server -> All Clients
    let (broadcast_tx, _) = broadcast::channel::<ChatMessage>(100);

    // Spawn Server
    let server_handle = tokio::spawn(server::run_server(
        server_rx,
        broadcast_tx.clone(),
    ));

    // Spawn Clients
    let client1 = tokio::spawn(client::run_client(
        "Akshat".to_string(),
        server_tx.clone(),
        broadcast_tx.subscribe(),
    ));

    let client2 = tokio::spawn(client::run_client(
        "Nitin".to_string(),
        server_tx.clone(),
        broadcast_tx.subscribe(),
    ));

    let client3 = tokio::spawn(client::run_client(
        "Karan".to_string(),
        server_tx,
        broadcast_tx.subscribe(),
    ));

    // Wait for all clients
    let _ = tokio::join!(client1, client2, client3);

    println!("\nAll clients disconnected.");

    // Close the server
    server_handle.abort();

    println!("Server stopped.");
}