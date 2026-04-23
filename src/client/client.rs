use mdns_sd::{ServiceDaemon, ServiceEvent};
use std::time::Duration;

fn main() -> std::io::Result<()> {
    let mdns = ServiceDaemon::new().expect("Failed to create daemon");

    let service_type = "_boxbiter._tcp.local.";
    let receiver = mdns.browse(service_type).expect("Failed to browse");
    let timeout = Duration::from_secs(10);

    loop {
        match receiver.recv_timeout(timeout) {
            Ok(ServiceEvent::ServiceResolved(info)) => {
                println!("Resolved: {}", info.get_fullname());
                println!("  Hostname : {}", info.get_hostname());
                println!("  Addresses: {:?}", info.get_addresses());
                println!("  Port     : {}", info.get_port());
                println!("  Properties: {:?}", info.get_properties());
                break;
            }
            Ok(other_event) => {
                println!("Received other event: {:?}", other_event);
            }
            Err(_) => {
                println!("Timed out after {}s waiting for service.", timeout.as_secs());
                break;
            }
        }
    }

    mdns.shutdown().unwrap();

    Ok(())
}
