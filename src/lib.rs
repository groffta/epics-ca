// Re-exports
pub mod protocol;
pub mod repeater;
pub mod client;
pub mod server;
pub use client::Client;
pub use server::Server;

// Imports

// EPICS Channel Access Protocol Version
pub const MAJOR_PROTOCOL_VERSION: u16 = 4;
pub const MINOR_PROTOCOL_VERSION: u16 = 11;

// Port Number Constants
const CA_PORT_BASE: u16 = 5056;
pub const CA_SERVER_PORT: u16 = CA_PORT_BASE + MAJOR_PROTOCOL_VERSION * 2;
pub const CA_REPEATER_PORT: u16 = CA_PORT_BASE + MAJOR_PROTOCOL_VERSION *2 + 1;

// Other Constants
const CA_SERVER_BEACON_MAX_PERIOD: f64 = 15.0;
const CA_REPEATER_CLIENT_CHECK_PERIOD: f64 = 1.0;
const LOCALHOST_U32: u32 = 0x7F000001;



#[cfg(test)]
mod tests {
    use super::*;
    use std::net::UdpSocket;

    #[test]
    fn client_registration() {
        std::env::set_var("RUST_LOG", "trace");
        pretty_env_logger::init();

        let client = client::Client::new().unwrap();
        std::thread::sleep(std::time::Duration::from_millis(10));

        assert!(client.is_registered());
    }
}

