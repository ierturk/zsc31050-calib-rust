// Copyright (c) 2025, Ibrahim Erturk <me@erturk.me>, ErturkMe
// Licensed under the BSD 3-Clause License.
// See the LICENSE file in the project root for more information.

use tracing::Level;
use tracing_subscriber::{FmtSubscriber, fmt::format::FmtSpan};

use common_lib::types::*;
use rbic1_dll_svc::dds_rpc_service::DdsRpcService;
use rbic1_dll_svc::invoker::Invoker;

#[tokio::main]
async fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::ERROR)
        .with_span_events(FmtSpan::ACTIVE)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("setting default subscriber failed");

    let invoker = Invoker::new().await;

    match invoker.invoke(RequestType::DLLVersion).await {
        Ok(reply) => println!("Received reply: {:?}", reply),
        Err(e) => eprintln!("Failed to invoke: {:?}", e),
    }
}
