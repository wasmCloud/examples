use anyhow::{anyhow, Result};
use clap::{Parser, Subcommand};
use nkeys::KeyPair;
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, sync::Arc, time::Duration};
use wasmbus_rpc::{async_nats, core::WasmCloudEntity};

const RPC_TIMEOUT: Duration = Duration::from_millis(2000);
const FAKE_HOST_ID: &str = "N0000000000000000000000000000000000000000000000000000000";

#[derive(Serialize, Deserialize)]
struct PubMessage {
    subject: String,
    message: Vec<u8>,
}

#[derive(Parser)]
struct Args {
    /// Nats address
    #[arg(short, long, default_value = "127.0.0.1:4222")]
    nats: String,

    /// Lattice prefix
    #[arg(short, long, default_value = "default")]
    lattice: String,

    #[command(subcommand)]
    command: Command,

    /// Host seed key (aka cluster seed) - used for signing invocations.
    /// If not specified, uses the value from environment WASMCLOUD_CLUSTER_SEED
    #[arg(short, long)]
    seed: Option<String>,
}

#[derive(Subcommand)]
enum Command {
    Sub(SubCmd),
}

#[derive(Parser)]
struct SubCmd {
    /// public key of actor - must already be linked to wasi-messaging provider
    #[arg(short, long)]
    actor: String,

    /// public key of wasi-messaging provider
    #[arg(short, long)]
    provider: String,

    /// subscription topic
    #[arg(short, long)]
    subject: String,

    #[arg(short, long, default_value = "default")]
    link_name: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    env_logger::try_init().ok();

    if let Err(e) = run(args).await {
        eprintln!("error: {}", e);
        std::process::exit(1);
    }
}

async fn run(args: Args) -> Result<()> {
    match args.command {
        Command::Sub(ref sub) => {
            send_subscribe(&args, sub).await?;
        }
    }
    Ok(())
}

/// send subscribe
/// Tests that capability provider responds to rpc message for consumer.subscribe
async fn send_subscribe(args: &Args, sub: &SubCmd) -> Result<()> {
    let nc = async_nats::connect(&args.nats)
        .await
        .map_err(|e| anyhow!("nats connect error for uri:'{}': {e}", &args.nats))?;
    //let host_id = concat!(env!("CARGO_PKG_NAME"), "_", env!("CARGO_PKG_VERSION")).to_string();
    let host_id = FAKE_HOST_ID.to_string();
    let seed = match (args.seed.as_ref(), std::env::var("WASMCLOUD_CLUSTER_SEED")) {
        (Some(s), _) => s.to_string(),
        (_, Ok(s)) => s,
        _ => {
            return Err(anyhow!("host seed required! It can be set with either the '--seed' paramter or the environment variable WASMCLOUD_CLUSTER_SEED"));
        }
    };
    let host_key = Arc::new(KeyPair::from_seed(&seed)?);
    let client = wasmbus_rpc::rpc_client::RpcClient::new(nc, host_id, None, host_key);

    let origin = WasmCloudEntity {
        public_key: sub.actor.clone(),
        ..Default::default()
    };

    let target = WasmCloudEntity {
        public_key: sub.provider.clone(),
        contract_id: "wasmcloud:wasi:messaging".to_string(),
        link_name: sub.link_name.clone(),
    };

    let msg = wasmbus_rpc::common::Message {
        method: "Messaging.Consumer.subscribe",
        arg: Cow::Owned(rmp_serde::to_vec(&sub.subject).unwrap()),
    };

    client
        .send_timeout(origin, target, &args.lattice, msg, RPC_TIMEOUT)
        .await
        .map_err(|e| anyhow!("subscribing to {}: {e} ", &sub.subject))?;

    println!("Subscribed");

    Ok(())
}
