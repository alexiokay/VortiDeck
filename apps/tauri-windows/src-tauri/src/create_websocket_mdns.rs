use std::time::Duration;
use zeroconf::{MdnsService, ServiceType, TxtRecord};
use zeroconf::prelude::{TMdnsService, TTxtRecord};
use tokio::time::sleep;
use zeroconf::event_loop::TEventLoop; 
use log::{info, error}; 
use std::net::Ipv4Addr;
use std::net::SocketAddrV4;
use std::str::FromStr;
use std::sync::{Arc, Mutex};

// Assuming a `Context` type exists
#[derive(Default)]  // Derive the Default trait
struct Context;

pub async fn create_websocket_mdns(ip: String, port: u16) -> Result<(), String> {
    // Initialize logging only once
    env_logger::try_init().unwrap_or_default();

    // Start the mDNS advertisement
    if let Err(e) = advertise_service(ip, port).await {
        eprintln!("Failed to advertise service: {}", e);
        return Err(e.to_string());
    }

    Ok(()) // No infinite loop here
}

async fn advertise_service(ip: String, port: u16) -> Result<(), zeroconf::error::Error> {
    let service_name = "vortideck";
    let protocol = "tcp";
    let sub_types = vec!["subtype1", "subtype2"];

    let service_type = ServiceType::with_sub_types(service_name, protocol, sub_types)?;

    let mut service = MdnsService::new(service_type, port);

    // Set the TxtRecord
    let mut txt_record = TxtRecord::new();
    txt_record.insert("version", "1.0")?;
    txt_record.insert("description", "Vorticium mDNS Service")?;
    service.set_txt_record(txt_record);

    // Set service context
    let context: Arc<Context> = Arc::default(); // Simplified without Mutex
    service.set_context(Box::new(context));

    // Log service registration status
    service.set_registered_callback(Box::new(|result, _context| match result {
        Ok(registration) => info!("Service successfully registered: {:?}", registration),
        Err(e) => error!("Failed to register service: {}", e),
    }));
    let event_loop = service.register()?;
    match service.register() {
        Ok(event_loop) => {
            let event_loop = Arc::new(Mutex::new(event_loop));
        },
        Err(e) => {
            eprintln!("Failed to register service: {}", e);
            return Err(e);
        }
    }

    // Parse and log the service address
    let addr = SocketAddrV4::new(
        Ipv4Addr::from_str(&ip).map_err(|e| zeroconf::error::Error::from(format!("IP Parse Error: {}", e)))?, 
        port
    );
    println!(
        "Advertising service: {} on {}:{}",
        service_name, addr.ip(), addr.port()
    );

    // Run the mDNS advertisement in the background
    tokio::spawn(async move {
        loop {
            match event_loop.poll(Duration::from_secs(1)) {
                Ok(()) => (),
                Err(err) => {
                    eprintln!("mDNS poll error: {}", err);
                    break;
                }
            }
            tokio::task::yield_now().await;  // Yield control to other tasks

            // // Sleep to avoid tight polling loops
            // sleep(Duration::from_secs(10)).await;
        }
    });

    Ok(())
}
