use tokio::net::UdpSocket;
use tokio::sync::Barrier;
use std::sync::Arc;
use std::error::Error;
use serialization::{Message, serialize_message, deserialize_message};
use std::time::Instant;


#[inline(always)]
pub fn serialize_message_zero_copy(message: &Message) -> Vec<u8> {
    // Implement zero-copy serialization here (e.g., using a packed binary format)
    // This should minimize heap allocations and copying.
    bincode::serialize(message).unwrap() // Example, but implement your own zero-copy serializer
}

#[inline(always)]
pub fn deserialize_message_zero_copy(data: &[u8]) -> Message {
    bincode::deserialize(data).unwrap() // Example deserialization, replace with custom implementation
}

async fn send_test_message(socket: Arc<UdpSocket>, server_addr: &str, message: Message) -> Result<(), Box<dyn Error>> {
    let encoded = serialize_message_zero_copy(&message);
    socket.send_to(&encoded, server_addr).await?;

    let mut buf = vec![0; 1024];  // Reuse the buffer instead of allocating it each time
    let (len, _) = socket.recv_from(&mut buf).await?;
    let ack: Message = deserialize_message_zero_copy(&buf[..len]);

    println!("Received acknowledgment: {:?}", ack);
    Ok(())
}

#[tokio::test]
async fn test_multi_client() -> Result<(), Box<dyn Error>> {
    let server_addr = "127.0.0.1:12345";
    let client_count = 500;  // Number of clients to simulate

    let barrier = Arc::new(Barrier::new(client_count + 1));  // +1 for server synchronization
    let mut handles = vec![];

    let start_time = Instant::now(); // Measure time for benchmarking

    for i in 0..client_count {
        let barrier = barrier.clone();
        let socket = Arc::new(UdpSocket::bind("0.0.0.0:0").await?);  // Bind each client to a different port
        let message = Message {
            id: i as u32,
            content: format!("Hello from client {}", i),
            response_to: Some(0),
        };

        // Spawn each client, but control concurrency
        let handle = tokio::spawn(async move {
            barrier.wait().await;  // Synchronize the start of all clients
            if let Err(e) = send_test_message(socket, &server_addr, message).await {
                eprintln!("Client {} error: {:?}", i, e);
            }
        });

        handles.push(handle);
    }

    barrier.wait().await;  // Synchronize the start of all clients
    futures::future::join_all(handles).await;

    // Benchmarking output
    println!("Completed multi-client test in {:?}", start_time.elapsed());

    Ok(())
}