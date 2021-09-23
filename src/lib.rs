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



#[cfg(test)]
mod tests {
    use std::net::UdpSocket;

    #[test]
    fn test() {

    }
}

