use std::net::{
    UdpSocket,
    SocketAddr,
    IpAddr,
    Ipv4Addr,
};

use log::{info, error};


/// Initializes a new repeater or connects to an existing repeater
pub fn init() {
    // Check for existing repeater by attempting to bind to repeater port
    match UdpSocket::bind(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0,0,0,0,)), crate::CA_REPEATER_PORT)) {
        Ok(socket) => {
            // No repeater bound. Close socket and spawn a new repeater.
            drop(socket);
            spawn_repeater();
        }
        Err(_e) => {
            // Repeater is already bound. Perform repeater registration.
            register();
        }
    }
}

fn spawn_repeater() {
    info!("Spawning new repeater on 0.0.0.0:{}", crate::CA_REPEATER_PORT);
    todo!()
}

fn register() {
    info!("Registering with existing repeater at 127.0.0.1:{}", crate::CA_REPEATER_PORT);
    todo!()
}
