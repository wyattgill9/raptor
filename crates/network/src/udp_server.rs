use tokio::net::UdpSocket;
use tokio::sync::Barrier;
use std::sync::Arc;
use std::error::Error;
use serialization::{Message, serialize_message, deserialize_message};
use std::time::Instant;
use crate::udp_client::{deserialize_message_zero_copy, serialize_message_zero_copy};

pub async fn start_server() -> Result<(), Box<dyn Error>> {
    let socket = UdpSocket::bind("127.0.0.1:12345").await?;
    let socket = Arc::new(socket);
    let mut buf = vec![0; 1024];  // Reuse the buffer

    println!("Server listening on 127.0.0.1:12345...");

    loop {
        let (len, addr) = socket.recv_from(&mut buf).await?;
        let message: Message = deserialize_message_zero_copy(&buf[..len]);  // Deserialize with zero-copy

        let socket = Arc::clone(&socket);
        tokio::spawn(async move {
            if let Err(e) = handle_client(socket, addr, message).await {
                eprintln!("Error handling client {}: {}", addr, e);
            }
        });
    }
}

pub async fn handle_client(socket: Arc<UdpSocket>, addr: std::net::SocketAddr, message: Message) -> Result<(), Box<dyn Error>> {
    // Send acknowledgment in a highly optimized manner
    let ack_message = Message {
        id: message.id,
        content: "ACK".to_string(),
        response_to: Some(message.id),
    };

    let encoded = serialize_message_zero_copy(&ack_message);
    socket.send_to(&encoded, addr).await?;
    Ok(())
}