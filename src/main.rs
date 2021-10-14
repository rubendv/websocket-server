use std::{env};
use log::{info, error};

use std::net::TcpListener;
use std::thread::spawn;
use tungstenite::accept;
use tungstenite::Message::Text;

/// A WebSocket echo server
fn main () {
    let _ = env_logger::try_init();
    let addr = env::args().nth(1).unwrap_or_else(|| "127.0.0.1:8080".to_string());
    let server = TcpListener::bind(addr).unwrap();
    // TODO: figure out how to gracefully stop, probably requires async
    for stream in server.incoming() {
        // TODO: figure out how to use ? or and_then to avoid this pattern
        let stream = match stream {
            Err(e) => {
                error!("Error with incoming: {}", e);
                continue
            }
            Ok(stream) => stream
        };

        spawn (move || {
            let mut websocket = match accept(stream) {
                Err(e) => {
                    error!("Error accepting: {}", e);
                    return
                }
                Ok(websocket) => websocket
            };

            loop {
                match websocket.read_message() {
                    Err(e) => {
                        error!("Error: {}", e);
                        break
                    }
                    Ok(Text(msg)) => { 
                        info!("Received message {}", msg);
                        if let Err(e) = websocket.write_message(Text(msg)) {
                            error!("Failed to send: {}", e);
                            break
                        }
                    }
                    _ => ()
                };
            }
        });
    }
}