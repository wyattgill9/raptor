use std::error::Error;
use network::udp_server;  

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    udp_server::start_server().await
}