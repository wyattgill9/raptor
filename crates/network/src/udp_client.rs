use std::net::UdpSocket;

pub fn run_udp_client() -> std::io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:0")?; // Bind to a random local port
    socket.connect("127.0.0.1:8080")?;
    
    let message = b"Hello, server!";
    socket.send(message)?;

    let mut buf = [0; 1024];
    let amt = socket.recv(&mut buf)?;
    println!("Received response: {}", String::from_utf8_lossy(&buf[..amt]));

    Ok(())
}