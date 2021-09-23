use std::net::{
    UdpSocket,
    SocketAddr,
    IpAddr,
    Ipv4Addr,
};

use log::{info, error};


/// Initializes a new repeater or connects to an existing repeater and returns a bound UDP socket for receiving messages.
pub fn init() -> UdpSocket {
    // Check for existing repeater by attempting to bind to repeater port
    match UdpSocket::bind(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0,0,0,0,)), crate::CA_REPEATER_PORT)) {
        Ok(socket) => {
            // No repeater bound. Close socket and spawn a new repeater.
            drop(socket);
            spawn_repeater()
        }
        Err(_e) => {
            // A repeater is already bound. Perform repeater registration.
            register()
        }
    }
}

/// Spawns a new repeater on a seperate thread and returns a bopund UDP socket for receiving messages
fn spawn_repeater() -> UdpSocket {
    info!("Spawning new repeater on 0.0.0.0:{}", crate::CA_REPEATER_PORT);
    todo!()
}

/// Performs registration handshake with an existing local repeater and returns a bound UDP socket for receiving messages.
fn register() -> UdpSocket {
    info!("Registering with existing repeater at 127.0.0.1:{}", crate::CA_REPEATER_PORT);
    todo!()
}

pub struct Repeater {

}
