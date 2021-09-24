use std::net::{UdpSocket, SocketAddr};
use std::str::FromStr;
use std::time::Instant;
use crate::repeater;
use crate::protocol::{
    MessageHeader,
    Command,
};


#[derive(Debug)]
pub enum Error {
    IoError(String),
    RegistrationError(String),
}
impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::IoError(format!("{:?}",e))
    }
}


struct ServerRecord {
    tcp_address: SocketAddr,
    last_beacon_id: u32,
    last_beacon_timestamp: Instant,
}

pub struct Client {
    repeater_socket: UdpSocket,
}

impl Client {
    pub fn new() -> Result<Self, Error> {
        repeater::init();
        let mut instance = Self {
            repeater_socket: UdpSocket::bind("127.0.0.1:0")?,
        };

        instance.register()?;

        Ok(instance)
    }

    /// Registers client with the local repeater and waits for confirmation
    fn register(&mut self) -> Result<(), Error> {
        let registration_header = MessageHeader {
            command: Command::CA_REPEATER_REGISTER.into(),
            payload_size: 0,
            data_type: 0,
            data_count: 0,
            parameter_1: 0,
            parameter_2: crate::LOCALHOST_U32,  
        };

        // Send registration message
        if let Err(e) = self.repeater_socket.send_to(
            &registration_header.as_bytes(), 
            SocketAddr::from_str(format!("127.0.0.1:{}", crate::CA_REPEATER_PORT).as_str()).unwrap()
        ) {
            return Err(Error::IoError(format!("Could not send registration packet: {:?}", e)))
        }
        
        Ok(())
    }
}