use mdns_sd::{ServiceDaemon, ServiceInfo};
use std::net::IpAddr;

pub struct MdnsServer {
    daemon: ServiceDaemon,
}

impl MdnsServer {
    pub fn new() -> Self {
        let daemon = ServiceDaemon::new().expect("failed to create mDNS daemon");
        let monitor = daemon.monitor().expect("failed to monitor daemon");
        std::thread::spawn(move || {
            while let Ok(event) = monitor.recv() {
                if let mdns_sd::DaemonEvent::Error(e) = event {
                    eprintln!("mDNS daemon error: {e}");
                }
            }
        });

        Self { daemon }
    }

    pub fn register(
        &self,
        service_type: &str,
        service_name: &str,
        host_name: &str,
        ips: &[IpAddr],
        port: u16,
        properties: &[(&str, &str)],
    ) {
        let service = ServiceInfo::new(service_type, service_name, host_name, ips, port, properties)
            .expect("failed to create service info");
        self.daemon.register(service).expect("failed to register service");
    }

    pub fn shutdown(self) {
        self.daemon.shutdown().unwrap();
    }
}