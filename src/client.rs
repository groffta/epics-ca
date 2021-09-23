use std::net::{UdpSocket, SocketAddr};
use std::time::Instant;
use crate::repeater;


struct ServerRecord {
    tcp_address: SocketAddr,
    last_beacon_id: u32,
    last_beacon_timestamp: Instant,
}

pub struct Client {
    repeater_socket: UdpSocket,
    
}

impl Client {
    pub fn new() {
        let socket = repeater::init();
    }
}