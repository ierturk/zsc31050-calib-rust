use tokio::sync::mpsc;

use common_lib::types::RBIC1ServiceErrorType;
mod network;

slint::include_modules!();

pub fn main() -> Result<(), RBIC1ServiceErrorType> {
    let (tx, rx) = mpsc::channel(100);
    let (ui_tx, mut ui_rx) = mpsc::channel(100);

    match slint::spawn_local(async move {
        network::start_client(ui_tx, rx).await;
    }) {
        Ok(_) => println!("Successfully spawned start_client"),
        Err(e) => {
            eprintln!("Failed to spawn start_client: {:?}", e);
            return Err(RBIC1ServiceErrorType::AppFailedToStart);
        }
    }

    let app = match AppWindow::new() {
        Ok(app) => app,
        Err(e) => {
            eprintln!("Failed to create AppWindow: {:?}", e);
            return Err(RBIC1ServiceErrorType::AppFailedToStart);
        }
    };

    let tx_clone = tx.clone();

    app.on_request_increase_value({
        let app_handle = app.as_weak();
        move || {
            if let Some(app) = app_handle.upgrade() {
                app.set_counter(app.get_counter() + 1);
            } else {
                eprintln!("Failed to upgrade app_handle in on_request_increase_value");
            }
        }
    });

    app.on_get_dll_version({
        let ui_handle = app.as_weak();
        let tx_clone = tx_clone.clone();
        move || {
            let tx_clone = tx_clone.clone();

            match slint::spawn_local(async move {
                if let Err(e) = tx_clone.send("get-dll-version".to_string()).await {
                    eprintln!("Failed to send message: {:?}", e);
                }
            }) {
                Ok(_) => println!("Successfully spawned get-dll-version"),
                Err(e) => eprintln!("Failed to spawn get-dll-version: {:?}", e),
            }

            match ui_handle.upgrade() {
                Some(ui) => {
                    ui.set_version(slint::SharedString::from("Loading..."));
                }
                None => {
                    eprintln!("Failed to upgrade ui_handle in on_get_dll_version");
                }
            }
        }
    });

    let app_handle = app.as_weak();

    match slint::spawn_local(async move {
        while let Some(reply) = ui_rx.recv().await {
            println!("Received from Client: {}", reply);

            match app_handle.upgrade() {
                Some(app) => {
                    app.set_version(slint::SharedString::from(reply));
                }
                None => {
                    eprintln!("Failed to upgrade app_handle in ui_rx.recv");
                }
            }
        }
    }) {
        Ok(_) => println!("Successfully spawned ui_rx.recv"),
        Err(e) => {
            eprintln!("Failed to spawn ui_rx.recv: {:?}", e);
            return Err(RBIC1ServiceErrorType::AppFailedToStart);
        }
    }

    match app.run() {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!("Failed to run app: {:?}", e);
            Err(RBIC1ServiceErrorType::AppFailedToStart)
        }
    }
}
