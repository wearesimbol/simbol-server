use std::vec::Vec;
use std::rc::Rc;
use std::cell::RefCell;
use ws::{listen, Sender, Handler, Message, Handshake, CloseCode, Result};
use serde_json;
// use serde_json::{Value, Error};

#[derive(Serialize, Deserialize)]
struct MessageJSON {
    #[serde(rename = "type")]
    type_name: String,
    content: String,
    from: usize,
    to: usize,
}

type Connections = Rc<RefCell<Vec<Sender>>>;
struct Server {
    ws: Sender,
    connections: Connections,
}

impl Handler for Server {
    fn on_open(&mut self, _: Handshake) -> Result<()> {
        let open_message = MessageJSON {
            type_name: String::from("open"),
            content: format!(""),
            from: usize::from(self.ws.token()),
            to: usize::from(self.ws.token()),
        };

        match serde_json::to_string(&open_message) {
            Ok(s) => {
                let msg = Message::text(s);
                self.ws.send(msg)?;
            },
            Err(e) => println!("{:?}", e)
        };

        let mut connections = self.connections.borrow_mut();
        for connection in connections.iter() {
            let connected_message = MessageJSON {
                type_name: String::from("connected"),
                content: format!(""),
                from: usize::from(self.ws.token()),
                to: usize::from(connection.token()),
            };

            match serde_json::to_string(&connected_message) {
                Ok(s) => {
                    let msg = Message::text(s);
                    connection.send(msg)?;
                },
                Err(e) => println!("{:?}", e)
            };
        }

        connections.push(self.ws.clone());

        Ok(())
    }

    fn on_message(&mut self, msg: Message) -> Result<()> {
        println!("*********Server got message '{}'. ", msg);

        match serde_json::from_str::<MessageJSON>(&msg.into_text()?) {
            Ok(received_message) => {
                if received_message.type_name == String::from("signal") {
                    let connections = self.connections.borrow();
                    for connection in connections.iter() {
                        if usize::from(connection.token()) == received_message.to {
                            let signal_message = MessageJSON {
                                type_name: String::from("signal"),
                                content: received_message.content.to_owned(),
                                from: usize::from(self.ws.token()),
                                to: usize::from(connection.token()),
                            };

                            match serde_json::to_string(&signal_message) {
                                Ok(s) => {
                                    let msg = Message::text(s);
                                    connection.send(msg);
                                },
                                Err(e) => println!("{:?}", e)
                            };
                        }
                    }
                }
            },
            Err(e) => println!("{:?}", e)
        };

        Ok(())
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        let mut connections = self.connections.borrow_mut();
        let index = connections.iter().position(|i| i.token() == self.ws.token()).unwrap();
        connections.remove(index);

        for connection in connections.iter() {
            let disconnected_message = MessageJSON {
                type_name: String::from("disconnected"),
                content: format!(""),
                from: usize::from(self.ws.token()),
                to: usize::from(connection.token()),
            };

            match serde_json::to_string(&disconnected_message) {
                Ok(s) => {
                    let msg = Message::text(s);
                    connection.send(msg);
                },
                Err(e) => println!("{:?}", e)
            };
        }

        match code {
            CloseCode::Normal => println!("The client is done with the connection."),
            CloseCode::Away   => println!("The client is leaving the site."),
            _ => println!("The client encountered an error: {}", reason),
        }
    }
}

pub struct MultiVP {
    pub address: String,
    pub port: u16,
}

impl MultiVP {
    pub fn new(address: String, port: u16) -> MultiVP {
        MultiVP {
            address: address,
            port: port
        }
    }

    pub fn run_server(&self) {
        let connections = Connections::new(RefCell::new(Vec::with_capacity(10_000)));;
        listen(format!("{}:{}", self.address, self.port), move |out| {
            Server {
                ws: out,
                connections: connections.clone()
            }
        }).unwrap()
    }
}