use std::net::{TcpStream, ToSocketAddrs};
use std::time::Duration;

struct ScanConfig {
    target_ip: String,
    start_port: u16,
    end_port: u16,
}

fn scan_port<A: ToSocketAddrs>(address: A) -> bool {
    // Set a practical timeout so the scan doesn't hang on filtered ports
    let timeout = Duration::from_millis(200);
    
    // Attempt the TCP handshake
    match TcpStream::connect_timeout(&address.to_socket_addrs().unwrap().next().unwrap(), timeout) {
        Ok(_) => true,
        Err(_) => false,
    }
}

fn main() {
    println!("[+] Initializing Blazing-Fast Rust Port Scanner...");

    let config = ScanConfig {
        target_ip: String::from("127.0.0.1"),
        start_port: 20,
        end_port: 1024,
    };

    println!("[*] Targeting Host: {}", config.target_ip);
    println!("[*] Scanning port range {} to {}...", config.start_port, config.end_port);
    println!("------------------------------------------------");

    let mut open_ports_found = 0;

    for port in config.start_port..=config.end_port {
        let target = format!("{}:{}", config.target_ip, port);
        
        if scan_port(&target) {
            println!("[+] Port {} is OPEN", port);
            open_ports_found += 1;
        }
    }

    println!("------------------------------------------------");
    println!("[+] Scan complete. Found {} open ports.", open_ports_found);
}
