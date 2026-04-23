use mdns_sd::{ServiceDaemon, ServiceEvent};
use std::io;
use std::io::Write;

fn main() -> std::io::Result<()> {
    // Create a daemon
    let mdns = ServiceDaemon::new().expect("Failed to create daemon");

    // Browse for a service type.
    let service_type = "_boxbiter._tcp.local.";

    let receiver = mdns.browse(service_type).expect("Failed to browse");

    // Receive the browse events in sync or async. Here is
    // an example of using a thread. Users can call `receiver.recv_async().await`
    // if running in async environment.
    std::thread::spawn(move || {
        while let Ok(event) = receiver.recv() {
            match event {
                ServiceEvent::ServiceResolved(resolved) => {
                    println!("Resolved a new service: {}", resolved.fullname);
                }
                other_event => {
                    println!("Received other event: {:?}", &other_event);
                }
            }
        }
    });

    // Gracefully shutdown the daemon.
    let mut input = String::new();
    print!("Press Enter to continue...");
    io::stdout().flush()?; // make sure prompt appears before waiting
    io::stdin().read_line(&mut input)?;

    mdns.shutdown().unwrap();

    Ok(())
}
