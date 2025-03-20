use common_lib::types::RBIC1ServiceErrorType;

fn main() -> Result<(), RBIC1ServiceErrorType> {
    match test_gui_lib::main() {
        Ok(_) => {
            println!("Successfully spawned start_client");
            Ok(())
        }
        Err(e) => {
            eprintln!("Failed to spawn start_client: {:?}", e);
            Err(e)
        }
    }
}
