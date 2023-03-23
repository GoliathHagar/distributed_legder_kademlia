use std::collections::HashMap;
use std::error::Error;
use std::str;
use std::net::UdpSocket;
use std::sync::{Arc, mpsc, Mutex};
use std::sync::mpsc::Receiver;
use std::thread;
use std::thread::JoinHandle;
use crate::constants::fixed_sizes::{RESPONSE_TIMEOUT, UDP_STREAMING_BUFFER_SIZE};
use crate::dht::kademlia::KademliaDHT;
use crate::network::datagram::{Datagram, DatagramType};
use crate::network::key::Key;
use crate::network::node::Node;

#[derive(Clone, Debug)] pub struct Client {
    pub rpc: Arc<RpcSocket>,
}

#[derive(Clone, Debug)] pub struct Server {
    pub app: Arc<KademliaDHT>,
}

#[derive(Clone, Debug)]
pub struct RpcSocket{
    pub node : Arc<Node>,
    pub socket: Arc<UdpSocket>,
    pub await_response: Arc<Mutex<HashMap<String, mpsc::Sender<Option<Datagram>>>>>,
}

impl RpcSocket {
    pub fn new(node : Node) -> Self {
        let host = node.get_address();

        println!("[INFO] Initializing host node {:?}", &host);

        let socket = UdpSocket::bind(&host)
            .expect(format!("[FAILED] Server --> Error while binding UdpSocket to specified addr {}",
                            host).as_str());

        Self{
            node: Arc::new(node),
            socket: Arc::new(socket),
            await_response: Arc::new(Mutex::new(HashMap::new()))
        }

    }
}

impl Server{
    pub fn new(app : Arc<KademliaDHT>) -> Server {
        Self{
            app
        }
    }

    pub fn start_service(self) -> JoinHandle<()> {


        let app = self.app.clone();
        thread::spawn(move || {
            ;
            let mut buffer =  [0u8; UDP_STREAMING_BUFFER_SIZE];

            loop {
                let (size , src_addr) = match app.service.socket
                    .recv_from(&mut buffer){
                    Ok((sz, src)) => (sz, src),
                    Err(_) => {
                        eprintln!("[FAILED] Server --> Failed to receive data from peer");
                        continue;
                    }
                };

                let payload =
                    String::from(match str::from_utf8(&buffer[..size]){
                        Ok(utf) => utf,
                        Err(_) => {
                            eprintln!("[FAILED] Server --> Unable to parse string from received bytes");
                            continue;
                        }
                    });

                println!("{}", src_addr);

                let mut data : Datagram = match serde_json::from_str(&payload) {
                    Ok(d) =>   d,
                    Err(_) =>  {
                        eprintln!("[FAILED] Server --> Unable to decode string payload [{}]", payload.trim());
                        continue;
                    }
                };

                if data.destination != app.service.node.get_address() {
                    eprintln!("[WARNING] Server --> Destination address doesn't match node address, ignoring");
                    continue;
                }

                if data.source != src_addr.to_string() {
                    eprintln!("[WARNING] Server --> Source address doesn't match socket address, ignoring");
                    continue;
                }

                println!("[Payload] {:?}", data);

                data.source= src_addr.to_string();

                match data.data_type {
                    DatagramType::REQUEST => {
                        Server::request_handler(app.clone(), data)
                    }
                    DatagramType::RESPONSE => {
                        self.clone().response_handler(data)

                    }
                    DatagramType::KILL => {break;}
                }
            }
        })

    }

    fn reply(rpc : Arc<RpcSocket>, msg: &Datagram) {
        let encoded = serde_json::to_string(msg)
            .expect("[FAILED] Server --> Unable to serialize message");

        rpc.socket
            .send_to(&encoded.as_bytes(), &msg.destination)
            .expect("[FAILED] Server --> Error while sending message to specified address");
    }

    fn request_handler( app: Arc<KademliaDHT>,  payload: Datagram, ){
        thread::spawn(move || {
            let response : Datagram = KademliaDHT::handle_request(app.clone(),payload);

            Server::reply(app.service.clone(),&Datagram {
                token_id : response.token_id,
                data_type: DatagramType::RESPONSE,
                source:response.destination,
                destination: response.source,
                data: response.data
            });

        });

    }

    fn response_handler(self, payload: Datagram) {
        thread::spawn(move || {
            let app = self.app.clone();
            let mut await_response = app.service.await_response
                .lock()
                .expect("[FAILED] Server --> Failed to acquire lock on AwaitResponse");

            let token = payload.token_id.clone();

            let tmp = match await_response.get(&token) {
                Some(sender) => sender.send(Some(payload)),
                None => {
                    eprintln!(
                        "[WARNING] Server --> Unsolicited response received, ignoring..."
                    );
                    return;
                }
            };

            if let Ok(_) = tmp {
                await_response.remove(&token);
            }
        });

    }

}

impl Client{
    pub fn new(rpc : Arc<RpcSocket>) -> Client {
        Self{
            rpc
        }
    }
    pub fn make_request(self, payload: &Datagram ) -> Receiver<Option<Datagram>> { //Todo: Verify nullability @GoliathHagar
        let (sender, receiver) = mpsc::channel();

        let string_payload = serde_json::to_string(payload).unwrap();

        let mut await_response = self.rpc.await_response.lock()
            .expect("[FAILED] Client --> Failed to acquire mutex on AwaitResponse");

        await_response.insert(payload.token_id.clone(), sender.clone());

        let data = serde_json::to_string(payload).unwrap();//todo: verify null

        if let Err(_) = self.rpc.socket.send_to(&data.as_bytes(), &payload.destination){
            eprintln!("[FAILED] Client unable to send request")
        }


        //request timeout remove
        let rpc = self.rpc.clone();
        let token = payload.token_id.clone();

        thread::spawn(move || {
            thread::sleep(std::time::Duration::from_millis(RESPONSE_TIMEOUT));
            if let Ok(_) = sender.send(None) {
                let mut await_response = rpc.await_response
                    .lock()
                    .expect("[FAILED] Client --> Failed to acquire mutex on Pending");
                await_response.remove(&token);
            }

        });

        receiver

    }
}

