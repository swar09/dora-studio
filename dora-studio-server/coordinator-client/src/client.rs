use dora_core::topics::dora_coordinator_port_rpc;
use dora_message::cli_to_coordinator::CoordinatorControlClient;
use eyre::{Context, Result};
use std::net::IpAddr;
use tarpc::{client, tokio_serde};

/// Helper function to create the RPC client mapping directly to dora-coordinator
pub async fn connect_rpc(addr: IpAddr, control_port: u16) -> Result<CoordinatorControlClient> {
    let rpc_port = dora_coordinator_port_rpc(control_port);

    // 1. Establish the TCP connection using standard tokio-serde JSON transport
    let transport =
        tarpc::serde_transport::tcp::connect((addr, rpc_port), tokio_serde::formats::Json::default)
            .await
            .context("failed to connect tarpc transport to coordinator")?;

    // 2. Wrap it in a tarpc client config and spawn the driver
    let client = CoordinatorControlClient::new(client::Config::default(), transport).spawn();

    Ok(client)
}
