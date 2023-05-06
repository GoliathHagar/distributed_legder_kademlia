use crate::constants::fixed_sizes::RESPONSE_TIMEOUT;
use crate::network::rpc::Rpc;
use crate::network::datagram::{Datagram, DatagramType};
use crate::network::key::Key;
use crate::network::node::Node;
use crate::network::rpc_socket::RpcSocket;
use log::{debug, error};
use std::sync::mpsc::Receiver;
use std::sync::{mpsc, Arc};
use std::thread;
use std::time::SystemTime;

#[derive(Clone, Debug)]
pub struct Client {
    pub rpc: Arc<RpcSocket>,
}

impl Client {
    pub fn new(rpc: Arc<RpcSocket>) -> Client {
        Self { rpc }
    }

    pub fn make_call(self, payload: Rpc, dest: Node) -> Receiver<Option<Datagram>> {
        let (sender, receiver) = mpsc::channel();
        let source_addr = self.rpc.node.get_address();

        let token = Key::new(format!(
            "{}=>{}@{:?}",
            source_addr,
            dest.get_address(),
            SystemTime::now()
        ));

        let payload = Datagram {
            token_id: token.clone(),
            data_type: DatagramType::REQUEST,
            data: payload,
            destination: dest.get_address(),
            source: source_addr,
        };

        debug!("New request {:?}", payload);

        let mut await_response = match self.rpc.awaiting_response.lock() {
            Ok(d) => d,
            Err(_) => {
                error!("Failed to acquire lock");
                return receiver;
            }
        };

        await_response.insert(token.clone(), sender.clone());

        let data = match serde_json::to_string(&payload) {
            Ok(d) => d,
            Err(_) => {
                error!("Unable to serialize message");
                return receiver;
            }
        };

        if let Err(_) = self
            .rpc
            .socket
            .send_to(&data.as_bytes(), &payload.destination)
        {
            error!("Client unable to send request");
            return receiver;
        }

        //request timeout remove
        let rpc = self.rpc.clone();
        let token = payload.token_id.clone();

        thread::spawn(move || {
            thread::sleep(std::time::Duration::from_millis(RESPONSE_TIMEOUT));
            if let Ok(_) = sender.send(None) {
                let mut await_response = match rpc.awaiting_response.lock() {
                    Ok(l) => l,
                    Err(_) => {
                        error!("Failed to acquire lock");
                        return;
                    }
                };

                await_response.remove(&token);
            }
        });

        receiver
    }

    //Do not use in kademlia class
    pub fn datagram_request(self, payload: Datagram) -> Receiver<Option<Datagram>> {
        let (sender, receiver) = mpsc::channel();

        debug!("New request {:?}", payload);

        let mut await_response = match self.rpc.awaiting_response.lock() {
            Ok(d) => d,
            Err(_) => {
                error!("Failed to acquire lock");
                return receiver;
            }
        };

        await_response.insert(payload.token_id.clone(), sender.clone());

        let data = match serde_json::to_string(&payload) {
            Ok(d) => d,
            Err(_) => {
                error!("Unable to serialize message");
                return receiver;
            }
        };

        if let Err(_) = self
            .rpc
            .socket
            .send_to(&data.as_bytes(), &payload.destination)
        {
            error!("Client unable to send request");
            return receiver;
        }

        //request timeout remove
        let rpc = self.rpc.clone();
        let token = payload.token_id.clone();

        thread::spawn(move || {
            thread::sleep(std::time::Duration::from_millis(RESPONSE_TIMEOUT));
            if let Ok(_) = sender.send(None) {
                let mut await_response = match rpc.awaiting_response.lock() {
                    Ok(l) => l,
                    Err(_) => {
                        error!("Failed to acquire lock");
                        return;
                    }
                };

                await_response.remove(&token);
            }
        });

        receiver
    }
}
