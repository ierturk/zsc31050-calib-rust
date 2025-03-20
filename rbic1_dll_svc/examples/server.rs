// Copyright (c) 2025, Ibrahim Erturk <me@erturk.me>, ErturkMe
// Licensed under the BSD 3-Clause License.
// See the LICENSE file in the project root for more information.

use std::time::Duration;
use tokio::time::sleep;
use tracing::Level;
use tracing_subscriber::{FmtSubscriber, fmt::format::FmtSpan};

use rbic1_dll_svc::dds_rpc_service::DdsRpcService;
use rbic1_dll_svc::proxy::Proxy;

#[tokio::main]
async fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::WARN)
        .with_span_events(FmtSpan::ACTIVE)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let mut proxy = Proxy::new().await;

    loop {
        match proxy.run().await {
            Ok(_) => {
                println!("Proxy exited successfully");
                break;
            }
            Err(e) => {
                eprintln!("Proxy encountered an error: {:?}. Recreating...", e);
                sleep(Duration::from_secs(1)).await;
                proxy = Proxy::new().await;
            }
        }
    }
}
