// use tonic::transport::Server;
use std::net::SocketAddr;

pub async fn start_server(port: u16) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let _addr = SocketAddr::from(([0, 0, 0, 0], port));
    println!("OTLP gRPC listener would start here");
    // Server::builder()
    //     .add_service(...)
    //     .serve(addr)
    //     .await?;
    Ok(())
}
