use tokio::net::UdpSocket;
use network::udp_server;

#[tokio::test]
async fn test_client_server_communication() -> std::io::Result<()> {
    // Set up the server
    let server = tokio::spawn(async {
        udp_server::run_udp_server().await.unwrap();
    });

    // Set up the client
    let client = tokio::spawn(async {
        let socket = UdpSocket::bind("127.0.0.1:0").await.unwrap();
        let msg = b"Hello, server!";
        socket.send_to(msg, "127.0.0.1:8080").await.unwrap();

        let mut buf = vec![0; 1024];
        let (len, _addr) = socket.recv_from(&mut buf).await.unwrap();
        assert_eq!(&buf[..len], msg, "Received incorrect message");
    });

    let _ = tokio::try_join!(server, client);  // Run both tasks concurrently
    Ok(())
}