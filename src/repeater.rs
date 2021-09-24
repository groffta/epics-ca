use std::{convert::TryFrom, net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket}, str::FromStr, sync::mpsc::Sender};

use crate::protocol::{
    HEADER_SIZE,
    MessageHeader,
    Command,
};

use log::{info, warn, error, debug, trace};

/// Initializes a new repeater or connects to an existing repeater and returns a bound UDP socket for receiving messages.
pub fn init() {
    // Check for existing repeater by attempting to bind to repeater port
    if let Ok(socket) = UdpSocket::bind(SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0,0,0,0,)), crate::CA_REPEATER_PORT)) {
        // No repeater bound. Close socket and spawn a new repeater.
        drop(socket);
        spawn_repeater()
    }
}

/// Spawns a new repeater on a seperate thread listening for incoming registrations and server beacons. Blocks until repeater is ready to receive messages.
fn spawn_repeater() {
    info!("Spawning new repeater on 0.0.0.0:{}", crate::CA_REPEATER_PORT);

    let (tx, rx) = std::sync::mpsc::channel::<bool>();

    std::thread::spawn(move || {
        let mut repeater = Repeater::new("0.0.0.0");

        repeater.listen(tx);
    });

    rx.recv().unwrap();
}

#[derive(Debug)]
struct RegisteredClient {
    remote_address: SocketAddr,
    forward_socket: UdpSocket,
}

pub struct Repeater {
    socket: UdpSocket,
    registered_clients: Vec<RegisteredClient>,
}
impl Repeater {
    pub fn new(bind_addr: &str) -> Self {
        let socket = UdpSocket::bind(format!("{}:{}", bind_addr, crate::CA_REPEATER_PORT)).expect("Only a single repeater should be created per host");
        Self {
            socket,
            registered_clients: vec!()
        }
    }
    /// Begins receiving and processing CA_PROTO_RSRV_IS_UP and CA_REPEATER_REGISTER messages
    pub fn listen(&mut self, ready_sender: Sender<bool>) {
        // Only CA_PROTO_RSRV_IS_UP and CA_REPEATER_REGISTER should be received which consist of only a 16-byte header
        let mut buf = [0u8; HEADER_SIZE];

        ready_sender.send(true).unwrap();
        'recv: while let Ok((amt, src)) = self.socket.recv_from(&mut buf) {
            trace!("Received UDP Packet");
            // Validate IPv4
            if !src.is_ipv4() {
                warn!("IPv6 sockets are not supported");
                continue;
            }
            // Validate packet size
            if amt != 16 {
                warn!("Received packet with invalid size");
                continue;
            }

            // Parse packet into a MessageHeader
            let header = MessageHeader::from_bytes(&buf).expect("Buffer should be 16-bytes");

            // Process received message
            match Command::try_from(header.command) {
                // Forward server beacon to all registered clients
                Ok(Command::CA_PROTO_RSRV_IS_UP) => {
                    for client in &self.registered_clients {
                        if let Err(e) = client.forward_socket.send_to(&buf, client.remote_address) {
                            error!("Could not forward message to {:?}: {:?}", client.remote_address, e);
                            continue;
                        }
                    }
                },

                // Register client and send confirmation message
                Ok(Command::CA_REPEATER_REGISTER) => {
                    // Validate registration address matches source address
                    let addr_buf: [u8;4] = header.parameter_2.to_be_bytes();
                    let received_addr = Ipv4Addr::new(addr_buf[0], addr_buf[1], addr_buf[2], addr_buf[3]);
                    let src_addr: IpAddr = src.ip();

                    // Check if client is already registered
                    for client in &self.registered_clients {
                        if client.remote_address == src {
                            warn!("Client is already registered");
                            continue 'recv;
                        }
                    }   

                    if received_addr != src_addr {
                        warn!("Registration address does not match socket address!");
                        continue;
                    }

                    // Bind new UDP socket for communicating with client
                    let client_socket = match UdpSocket::bind("127.0.0.1:0"){
                        Ok(socket) => socket,
                        Err(e) => { 
                            error!("Could not create a new client UDP socket: {:?}", e);
                            continue;
                        }
                    };

                    // Convert local SocketAddr to octets
                    let local_addr_buf: [u8;4] = if let IpAddr::V4(ipv4addr) = self.socket.local_addr().expect("Repeater socket should have a SocketAddr").ip() {
                        ipv4addr.octets()
                    } else {
                        error!("Local socket should be IPv4!");
                        continue;
                    };

                    // Create confirmation message
                    let confirm_buf = MessageHeader {
                        command: Command::CA_REPEATER_CONFIRM.into(),
                        payload_size: 0,
                        data_type: 0,
                        data_count: 0,
                        parameter_1: 0,
                        parameter_2: u32::from_be_bytes(local_addr_buf),
                    };

                    // Send confirmation message
                    if let Err(e) = client_socket.send_to(&confirm_buf.as_bytes(), src) {
                        error!("Could not send registration confirmation to client: {:?}", e);
                        continue;
                    }

                    // Create new client record and add to registered_clients
                    let client = RegisteredClient {
                        remote_address: src,
                        forward_socket: client_socket,
                    };

                    info!("Registered new client: {:?}", client);
                    self.registered_clients.push(client);

                },
                Err(e) => {
                    warn!("Received invalid header command: {:?}", e);
                    continue;
                }
                _ => (),
            }

        }
    }
}
