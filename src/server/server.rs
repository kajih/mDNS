mod mdns;
mod settings;

use if_addrs::get_if_addrs;
use std::io;
use std::io::Write;
use std::net::IpAddr;

fn main() -> std::io::Result<()> {
    let settings = settings::load();
    let ips = resolve_iface_ips(&settings.iface_name);
    println!("IPs = {:?}", ips);

    let server = mdns::MdnsServer::new();
    server.register(
        "_boxbiter._tcp.local.",
        "_robox",
        &format!("{}.local.", local_hostname()),
        &ips,
        5200,
        &[("property_1", "test"), ("property_2", "1234")],
    );

    let mut input = String::new();
    print!("Press Enter to exit...");
    io::stdout().flush()?;
    io::stdin().read_line(&mut input)?;

    server.shutdown();
    Ok(())
}

fn resolve_iface_ips(iface_name: &str) -> Vec<IpAddr> {

    let matching: Vec<_> = get_if_addrs()
        .expect("failed to get network interfaces")
        .into_iter()
        .filter(|i| i.name.contains(iface_name) && !i.is_loopback())
        .collect();

    if matching.is_empty() {
        panic!("no interface matching '{iface_name}' found");
    }

    let mut unique_names: Vec<&str> = matching.iter().map(|i| i.name.as_str()).collect();
    unique_names.sort_unstable();
    unique_names.dedup();
    if unique_names.len() > 1 {
        let list = unique_names
            .iter()
            .map(|n| {
                let ips = matching
                    .iter()
                    .filter(|i| i.name == *n)
                    .map(|i| i.ip().to_string())
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("  {n} ({ips})")
            })
            .collect::<Vec<_>>()
            .join("\n");
        panic!("more than one interface matching '{iface_name}' found, be more specific:\n{list}");
    }

    matching.iter().map(|i| i.ip()).collect()
}

fn local_hostname() -> String {
    std::env::var("COMPUTERNAME")
        .or_else(|_| std::env::var("HOSTNAME"))
        .unwrap_or_else(|_| "unknown-host".to_string())
        .to_lowercase()
}