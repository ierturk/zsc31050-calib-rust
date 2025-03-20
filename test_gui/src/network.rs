use tokio::sync::mpsc::{Receiver, Sender};

use common_lib::types::*;
use rbic1_dll_svc::dds_rpc_service::DdsRpcService;
use rbic1_dll_svc::invoker::Invoker;

pub async fn start_client(ui_tx: Sender<String>, mut rx: Receiver<String>) {
    let mut invoker = Invoker::new().await; // Create the initial Invoker

    while let Some(message) = rx.recv().await {
        println!("Received from Slint: {}", message);

        match invoker.invoke(RequestType::DLLVersion).await {
            Ok(reply) => {
                println!("Received reply: {:?}", reply);
                match reply {
                    ReplyType::DLLVersion { version: v } => {
                        println!("DLL Version: {}", v);
                        if let Err(e) = ui_tx.send(v).await {
                            eprintln!("Failed to send DLL version: {:?}", e);
                        }
                    }
                    _ => {
                        println!("Unknown");
                    }
                }
            }
            Err(e) => {
                eprintln!("Failed to invoke: {:?}. Restarting Invoker...", e);
                invoker = Invoker::new().await; // Recreate the Invoker on error
            }
        }
    }
}
