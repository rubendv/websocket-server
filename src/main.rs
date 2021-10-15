use std::{env};
use log::{info, error};

use std::net::{TcpListener, TcpStream};
use std::thread::spawn;
use tungstenite::accept;
use tungstenite::Message::Text;

mod error;

use error::Error;

fn handle_stream(stream: TcpStream) -> Result<(), Error> {
    let mut websocket = accept(stream).map_err(|e| Error::new("failed to accept", e))?;

    loop {
        if let Text(msg) = websocket.read_message()? {
            info!("Received message {}", msg);
            websocket.write_message(Text(msg))?
        }
    }
}

/// A WebSocket echo server
fn server () -> Result<(), Error> {
    let _ = env_logger::try_init();
    let addr = env::args().nth(1).unwrap_or_else(|| "127.0.0.1:8080".to_string());
    let server = TcpListener::bind(&addr).map_err(|e| Error::new("failed to bind", e))?;

    info!("Started listening for incoming connections on {}", addr);

    // TODO: figure out how to gracefully stop, probably requires async
    let streams = server.incoming().filter_map(|s| {
        match s {
            Ok(stream) => { Some(stream) }
            Err(e) => {
                error!("Error with incoming: {}", e);
                None
            }
        }
    });

    for stream in streams {
        spawn (move || {
            if let Err(e) = handle_stream(stream) {
                error!("While handling stream: {}. Closing connection.", e);
            }
        });
    }

    Ok(())
}

fn main() {
    if let Err(e) = server() {
        error!("{}", e)
    }
}