use std::convert::TryFrom;
use std::net::{UdpSocket, SocketAddr};
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::{channel, Sender, Receiver};
use std::time::{Instant, Duration};
use crate::{repeater, server};
use crate::protocol::{
    MessageHeader,
    Command,
};

use log::{info, warn, error, debug, trace};

const UPDATE_PERIOD: f64 = 0.5;

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
    repeater_socket: Arc<Mutex<UdpSocket>>,
    registered: Arc<Mutex<bool>>,
    server_list: Arc<Mutex<Vec<ServerRecord>>>,
    process_stopper: Option<Sender<bool>>,
    update_stopper:  Option<Sender<bool>>,
}

impl Client {
    pub fn new() -> Result<Self, Error> {
        repeater::init();
        //std::thread::sleep(std::time::Duration::from_millis(10));
        let mut instance = Self {
            repeater_socket: Arc::new(Mutex::new(UdpSocket::bind("127.0.0.1:0")?)),
            registered: Arc::new(Mutex::new(false)),
            server_list: Arc::new(Mutex::new(vec!())),
            process_stopper: None,
            update_stopper: None,
        };

        instance.register()?;

        // Start processing threads
        instance.start_processing_packets();
        instance.start_processing_update();

        Ok(instance)
    }

    /// Returns true if the client has registered with the repeater and received a confirmation message.
    pub fn is_registered(&self) -> bool {
        *self.registered.lock().unwrap()
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
        if let Err(e) = self.repeater_socket.lock().unwrap().send_to(
            &registration_header.as_bytes(), 
            SocketAddr::from_str(format!("127.0.0.1:{}", crate::CA_REPEATER_PORT).as_str()).unwrap()
        ) {
            return Err(Error::IoError(format!("Could not send registration packet: {:?}", e)))
        }
        debug!("Registration message sent");
        Ok(())
    }

    /// Spawns a new thread that handles incoming datagrams.
    pub fn start_processing_packets(&mut self) {
        let (tx, rx) = channel::<bool>();

        let socket = self.repeater_socket.clone();
        let registered = self.registered.clone();

        std::thread::spawn(move || {
            let mut packet_buf = [0u8; crate::protocol::HEADER_SIZE];

            // Loop over incoming UDP packets
            while let Ok((amt, src)) = socket.lock().unwrap().recv_from(&mut packet_buf) {
                // Parse received packet into a message header
                if let Ok(header) = MessageHeader::from_bytes(&packet_buf) {
                    match Command::try_from(header.command) {
                        Ok(Command::CA_REPEATER_CONFIRM) => {
                            // Store repeater confirmation
                            debug!("Received registration confirmation from repeater");
                            *registered.lock().unwrap() = true;
                        },
                        Ok(Command::CA_PROTO_RSRV_IS_UP) => {
                            // Update server list
                            trace!("Received server beacon");
                            todo!()
                        }
                        Err(e) => {
                            error!("Error receiving UDP packet: {:?}", e);
                            continue;
                        }
                        _ => {
                            warn!("Client received unsupported message command");
                            continue;
                        }
                    }
                }

                // Check for stop signal
                if let Ok(stop) = rx.try_recv() {
                    if stop { break; }
                }
            }0
        });

        self.process_stopper = Some(tx);
    }

    /// Sends a stop message via an mpsc channel to the processing thread
    pub fn stop_processing_packets(&mut self) {
        if let Some(sender) = &self.process_stopper {
            if let Err(e) = sender.send(true) {
                error!("Could not stop packet processing thread: {:?}", e)
            }
        }
    }

    /// Spawns a new thread that handles periodic tasks like checking server timeouts.
    pub fn start_processing_update(&mut self) {
        let (tx, rx) = channel::<bool>();

        let servers = self.server_list.clone();
        
        std::thread::spawn(move || {
            loop {
                // Remove expired server records
                let mut lock = servers.lock().unwrap();
    
                lock.retain(|server_record| {
                    Instant::now() - server_record.last_beacon_timestamp < Duration::from_secs_f64(crate::CA_SERVER_BEACON_MAX_PERIOD*2.0)
                });
    
    
                // Check for stop signal
                if let Ok(stop) = rx.try_recv() {
                    if stop { break; }
                }

                // Sleep for update period
                std::thread::sleep(Duration::from_secs_f64(UPDATE_PERIOD))

            }
        });

        self.update_stopper = Some(tx);
    }

    /// Sends a stop message via an mpsc channel to the update thread
    pub fn stop_processing_update(&mut self) {
        if let Some(sender) = &self.update_stopper {
            if let Err(e) = sender.send(true) {
                error!("Could not update thread: {:?}", e)
            }
        }
    }
}