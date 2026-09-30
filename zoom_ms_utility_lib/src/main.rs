

use zoom_ms::{ConnectionStatus, ZoomClient};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .try_init();

    let client = ZoomClient::new();

    // Connect
    match client.connect() {
        Ok(ConnectionStatus::Connected) => println!("Connected!"),
        Ok(ConnectionStatus::AlreadyConnected) => println!("Already connected!"),
        Ok(ConnectionStatus::NoDeviceFound) => println!("No device found."),
        Err(e) => println!("Error: {}", e),
    }

    if client.is_connected() {
        // Scan library
        client.scan_library()?;

        // List patches
        client.list_library();

        // Select a patch
        client.select_patch(0)?;

        // Toggle a slot
        client.toggle_slot(0)?;

        // Edit a parameter
        client.edit_parameter(0, 2, 80)?;
    }

    Ok(())
}
