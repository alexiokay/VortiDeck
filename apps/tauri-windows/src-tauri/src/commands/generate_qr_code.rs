use base64::engine::general_purpose::STANDARD as Base64Standard;
use base64::Engine;
use image::{DynamicImage, ImageFormat, Luma};
use qrcode::QrCode;
use std::env;
use std::io::Cursor;
use tauri::command;
use uuid::Uuid;

fn get_hostname() -> String {
    env::var("COMPUTERNAME") // Windows
        .or_else(|_| env::var("HOSTNAME")) // Unix-like systems
        .unwrap_or_else(|_| "unknown-host".to_string())
}

#[tauri::command]
pub fn generate_qr_code(data: Option<String>) -> Result<String, String> {
    let ip_address = "127.0.0.1"; // Replace with dynamic IP retrieval if needed
    let secret_key = Uuid::new_v4().to_string();
    let hostname = get_hostname();
    let websocket_url = format!("ws://{}:9001", ip_address);

    let custom_data = data.unwrap_or_else(|| "default-message".to_string());

    let qr_data = format!(
        "{{\"ip\":\"{}\",\"key\":\"{}\",\"ws\":\"{}\",\"host\":\"{}\",\"data\":\"{}\"}}",
        ip_address, secret_key, websocket_url, hostname, custom_data
    );

    let code = QrCode::new(&qr_data).map_err(|e| e.to_string())?;
    let image = code.render::<Luma<u8>>().build();
    let dynamic_image = DynamicImage::ImageLuma8(image);

    let mut buf = Cursor::new(Vec::new());
    dynamic_image
        .write_to(&mut buf, ImageFormat::Png)
        .map_err(|e| e.to_string())?;
    let base64_string = Base64Standard.encode(buf.get_ref());

    Ok(base64_string)
}
