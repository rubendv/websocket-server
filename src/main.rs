use std::{env};
use log::{info, error};

use std::sync::{Arc, Mutex};
use std::net::{TcpListener, TcpStream};
use std::thread::spawn;
use tungstenite::accept;
use tungstenite::{Message, WebSocket};
use rand::Rng;
use std::{thread, time};

mod error;

use error::{Error, CustomResult};

fn handle_stream(stream: TcpStream, simulation_state: Arc<Mutex<SimulationState>>) -> Result<(), Error> {
    let mut websocket: WebSocket<TcpStream> = accept(stream).custom_err("failed to accept")?;

    loop {
        let price = {
            simulation_state.lock().unwrap().price
        };
        websocket.write_message(Message::text(price.to_string())).unwrap();
        thread::sleep(time::Duration::from_millis(1000));
    }
}

struct SimulationState {
    price: f64
}

fn simulation() -> Arc<Mutex<SimulationState>> {
    let simulation_state = Arc::new(Mutex::new(SimulationState { price: 0.0 }));
    let simulation_state_return = simulation_state.clone();
    
    spawn(move || {
        let mut rng = rand::thread_rng();
        loop {
            {
                simulation_state.lock().unwrap().price += rng.gen_range(-1.0..1.0);
            }
            thread::sleep(time::Duration::from_millis(100));
        }
    });

    simulation_state_return
}

/// A WebSocket echo server
fn server () -> Result<(), Error> {
    let _ = env_logger::try_init();
    let addr = env::args().nth(1).unwrap_or_else(|| "127.0.0.1:8080".to_string());
    let server = TcpListener::bind(&addr).custom_err("failed to bind")?;
    let simulation_state = simulation();

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
        let simulation_state = simulation_state.clone();
        spawn (move || {
            if let Err(e) = handle_stream(stream, simulation_state) {
                error!("While handling stream: {}. Closing connection.", e);
            }
        });
    }

    Ok(())
}

fn main() -> Result<(), ()> {
    if let Err(e) = server() {
        error!("{}", e);
        Err(())
    } else {
        Ok(())
    }
}