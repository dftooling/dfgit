
use std::net::TcpStream;

use websocket::{sync::Client, ClientBuilder, Message, OwnedMessage};

use crate::template::{self, Template};

pub struct CCAPI {
    client: Client<TcpStream>,
}

impl CCAPI {
    pub fn connect() -> CCAPI {
        let client = ClientBuilder::new("ws://localhost:31375")
            .unwrap()
            .connect_insecure()
            .unwrap();
        return CCAPI{
            client
        };
    }

    pub fn wait_for_auth(&mut self) {
        self.client.send_message(&Message::text("scopes read_plot write_code"));
        loop {
            let message = self.client.recv_message().unwrap(); // Recieve message
            match message {
                OwnedMessage::Text(string) => { // Text data
                    // If the string contains 'auth', it means we have been authorised.
                    if string.contains("auth") { 
                        break;
                    }
                }
                _ => {} // Other data
            }
        }
    }

    pub fn scan(&mut self) -> Vec<Template> {
        self.client.send_message(&Message::text("scan")).unwrap(); // Send message

        loop {
            let message = self.client.recv_message().unwrap(); // Recieve message
            match message {
                OwnedMessage::Text(string) => { // Text data
                    
                    let mut vec = Vec::new();
                    let slices: Vec<&str> = string.split("\n").collect();
                    for d in slices {
                        vec.push(Template::new(String::from(d)));
                    }
                    return vec;

                }
                _ => {} // Other data
            }
        }
    }

    pub fn place(&mut self, templates: Vec<Template>) {
        self.client.send_message(&Message::text("place")).unwrap(); // Send init message
        for template in templates {
            let data = template.get_data(); // get data (funny)
            let message = format!("place {}", data); // Format message
            self.client.send_message(&Message::text(message)).unwrap(); // Send message
        }
        self.client.send_message(&Message::text("place go")).unwrap(); // Send go message
    }

    pub fn clear(&mut self) {
        self.client.send_message(&Message::text("clear")).unwrap(); // Send clear message
    }

    pub fn close(&mut self) {
        self.client.shutdown().unwrap();
    }
}