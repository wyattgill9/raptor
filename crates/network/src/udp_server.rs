use tokio::net::UdpSocket;
use std::sync::Arc;
use std::error::Error;
use serialization::{Message, serialize_message, deserialize_message};

pub async fn handle_client(socket: Arc<UdpSocket>, addr: std::net::SocketAddr, message: Message) -> Result<(), Box<dyn Error>> {
    println!("Received message from {}: {:?}", addr, message);

    // Send acknowledgment
    let ack_message = Message {
        id: message.id,
        content: "ACK".to_string(),
        response_to: Some(message.id),
    };
    let encoded = serialize_message(&ack_message)?;
    socket.send_to(&encoded, addr).await?;
    Ok(())
}

pub async fn start_server() -> Result<(), Box<dyn Error>> {
    let socket = UdpSocket::bind("127.0.0.1:12345").await?;
    let socket = Arc::new(socket);

    println!("Server listening on 127.0.0.1:12345...");

    loop {
        let mut buf = [0; 1024];
        let (len, addr) = socket.recv_from(&mut buf).await?;
        let message: Message = deserialize_message(&buf[..len])?;  // Deserialize message

        let socket = Arc::clone(&socket);
        tokio::spawn(async move {
            if let Err(e) = handle_client(socket, addr, message).await {
                eprintln!("Error handling client {}: {}", addr, e);
            }
        });
    }
}
