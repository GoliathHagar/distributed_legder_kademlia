use std::sync::{Arc, mpsc};
use std::sync::mpsc::Receiver;
use std::thread;
use crate::constants::fixed_sizes::RESPONSE_TIMEOUT;
use crate::network::datagram::Datagram;
use crate::network::rpc_socket::RpcSocket;

#[derive(Clone, Debug)] pub struct Client {
    pub rpc: Arc<RpcSocket>,
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
