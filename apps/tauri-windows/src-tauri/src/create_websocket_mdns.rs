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
    // Initialize logging system
    env_logger::init();


    // Run the mDNS advertisement
    if let Err(e) = advertise_service(ip, port).await {
        eprintln!("Failed to advertise service: {}", e);
    }

    // Keep the main function alive to allow service to continue advertising
    loop {
        sleep(Duration::from_secs(10)).await; // Sleep for 60 seconds in a loop
    }
}

async fn advertise_service(ip: String, port: u16) -> Result<(), zeroconf::error::Error> {
    let service_name = "vortideck"; // Service name without domain part
    let protocol = "tcp"; // Protocol type
    let sub_types = vec!["subtype1", "subtype2"]; // Subtypes (for example)
    
    // Use ServiceType::with_sub_types to define the service
    let service_type = ServiceType::with_sub_types(service_name, protocol, sub_types)?;

    let mut service = MdnsService::new(service_type, port);



    // Create and set a TxtRecord
    let mut txt_record = TxtRecord::new();
    txt_record.insert("version", "1.0")?; // Optional version
    txt_record.insert("description", "Vorticium mDNS Service")?;
    service.set_txt_record(txt_record);

    // Context: Assuming Context type is used for storing some state
    let context: Arc<Mutex<Context>> = Arc::default();
    service.set_context(Box::new(context));

    // Register callback for when the service is registered
    service.set_registered_callback(Box::new(|result, _context| {
        match result {
            Ok(registration) => {
                info!("Service successfully registered: {:?}", registration);
            }
            Err(e) => {
                error!("Failed to register service: {}", e);
            }
        }
    }));

    // Register the service and get the event loop
    let event_loop = service.register()?;

    // Parse the IP address manually and handle errors
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
            match event_loop.poll(Duration::from_secs(5)) {
                Ok(()) => (), // Successful poll, continue
                Err(err) => {
                    eprintln!("mDNS poll error: {}", err);
                    break; // Exit loop on error
                }
            }

            // Sleep for 1 second to prevent tight polling loop
            sleep(Duration::from_secs(1)).await;
        }
    });

    Ok(())
}
