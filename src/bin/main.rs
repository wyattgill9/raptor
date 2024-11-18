#[tokio::main]
async fn main() -> std::io::Result<()> {
    match network::udp_server::run_udp_server().await {
        Ok(_) => println!("UDP Server started successfully"),
        Err(e) => eprintln!("Error starting UDP server: {}", e),
    }
    Ok(())
}