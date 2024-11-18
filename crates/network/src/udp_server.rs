use tokio::net::UdpSocket;
use std::error::Error;

pub async fn run_udp_server() -> Result<(), Box<dyn Error>> {
    let socket = UdpSocket::bind("127.0.0.1:8080").await?;
    let mut buf = [0; 1024];

    loop {
        let (len, addr) = socket.recv_from(&mut buf).await?;
        println!("Received {} bytes from {:?}", len, addr);

        socket.send_to(&buf[..len], &addr).await?;
    }
}