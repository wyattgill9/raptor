use tokio::net::UdpSocket;
use std::error::Error;
use std::sync::Arc;
use serialization::{Message, serialize_message, deserialize_message};
use tokio::sync::Barrier;

async fn send_test_message(socket: Arc<UdpSocket>, server_addr: &str, message: Message) -> Result<(), Box<dyn Error>> {
    let encoded = serialize_message(&message)?;
    socket.send_to(&encoded, server_addr).await?;

    let mut buf = [0; 1024];
    let (len, _) = socket.recv_from(&mut buf).await?;
    let ack: Message = deserialize_message(&buf[..len])?;
    println!("Received acknowledgment: {:?}", ack);

    Ok(())
}

#[tokio::test]
async fn test_multi_client() -> Result<(), Box<dyn Error>> {
    let server_addr = "127.0.0.1:12345";
    let client_count = 100;  // Number of clients to simulate

    let barrier = Arc::new(Barrier::new(client_count + 1));  // +1 for server synchronization
    let mut handles = vec![];

    for i in 0..client_count {
        let barrier = barrier.clone();
        let socket = Arc::new(UdpSocket::bind("0.0.0.0:0").await?);  // Bind each client to a different port
        let message = Message {
            id: i as u32,
            content: format!("Hello from client {}", i),
            response_to: Some(0),
        };

        // Spawn each client
        let handle = tokio::spawn(async move {
            barrier.wait().await;  // Wait for all clients to be ready
            match send_test_message(socket, &server_addr, message).await {
                Ok(_) => println!("Client {} finished successfully", i),
                Err(e) => eprintln!("Client {} error: {:?}", i, e),
            }
        });

        handles.push(handle);
    }

    barrier.wait().await;  // Synchronize the start of all clients
    let results = futures::future::join_all(handles).await;

    for result in results {
        result.unwrap();  // Ensure no errors occurred
    }

    Ok(())
}
