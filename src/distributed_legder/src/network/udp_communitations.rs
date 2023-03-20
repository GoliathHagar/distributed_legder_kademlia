
use std::str;
use std::net::UdpSocket;
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;
use crate::constants::fixed_sizes::UDP_STREAMING_BUFFER_SIZE;
use crate::dht::kademlia::KademliaDHT;
use crate::network::node::Node;

#[derive(Clone, Debug)]
pub struct Server{
    pub node : Node,
    pub socket: Arc<UdpSocket>,
    // pub pending: Arc<Mutex<HashMap<Key, mpsc::Sender<Option<String>>>>>,

}

#[derive(Clone, Debug)]
pub struct Client{
    pub node : Node,
    // pub pending: Arc<Mutex<HashMap<Key, mpsc::Sender<Option<String>>>>>,

}

impl Server {
    pub fn new(node : Node) -> Self {
        let host = node.get_address();

        println!("initializing host: {:?}", &host);

        let socket = UdpSocket::bind(&host)
            .expect(format!("[FAILED] Server --> Error while binding UdpSocket to specified addr {}",
                            host).as_str());

        Self{
            node,
            socket: Arc::new(socket),
           // pending: Arc::new(Mutex::new(HashMap::new()))
        }


    }
    pub fn start_service(server :Server, kad : KademliaDHT) -> JoinHandle<()> {
        thread::spawn(move || {
            let mut buffer =  [0u8; UDP_STREAMING_BUFFER_SIZE];

            loop {
                let (size , src_addr) = server
                    .socket
                    .recv_from(&mut buffer)
                    .expect("[FAILED] Server Service --> Failed to receive data from peer");

                let payload =
                    String::from(str::from_utf8(&buffer[..size]).expect(
                        "[FAILED] Server Service --> Unable to parse string from received bytes",
                    ));

                println!("[Payload] {}", payload);


                let resp = kad.handle_request(&payload );

                server.socket.send_to(resp.as_bytes(), src_addr).expect("Error sending dat");

                if payload.contains("stop"){
                    break;
                }
            }
        })

    }

}

impl Client{

    pub fn make_request(source: Arc<UdpSocket>, destination: Node, payload: String ){


        source.send_to(payload.as_bytes(), destination.get_address()).expect("Error sending dat");

    }
}

