use futures::prelude::*;
use libp2p::{ping, swarm::{SwarmEvent}, Multiaddr};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufReader, BufWriter};

#[derive(Serialize, Deserialize, Debug)]
struct Note {
    id: u32,
    title: String,
    content: String,
}

fn save_note(note: &Note) -> io::Result<()> {
    let file = File::create("note.json")?;
    let writer = BufWriter::new(file);
    serde_json::to_writer(writer, note)?;
    Ok(())
}

fn load_note() -> io::Result<Note> {
    let file = File::open("note.json")?;
    let reader = BufReader::new(file);
    let note = serde_json::from_reader(reader)?;
    Ok(note)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Example storage test
    let note = Note {
        id: 1,
        title: "Test Note".to_string(),
        content: "Hello DOC - Physics efficient storage".to_string(),
    };
    save_note(&note)?;
    let loaded = load_note()?;
    println!("Loaded note: {:?}", loaded);

    // Existing P2P code (integrated)
    let mut swarm = libp2p::SwarmBuilder::with_new_identity()
        .with_tokio()
        .with_tcp(
            libp2p::tcp::Config::default(),
            libp2p::noise::Config::new,
            libp2p::yamux::Config::default,
        )?
        .with_behaviour(|_| ping::Behaviour::default())?
        .with_swarm_config(|cfg| cfg.with_idle_connection_timeout(std::time::Duration::from_secs(30)))
        .build();

    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

    if let Some(addr) = std::env::args().nth(1) {
        let remote: Multiaddr = addr.parse()?;
        swarm.dial(remote)?;
        println!("Dialed {addr}");
    }

    loop {
        match swarm.select_next_some().await {
            SwarmEvent::NewListenAddr { address, .. } => println!("Listening on {address:?}"),
            SwarmEvent::Behaviour(event) => println!("{event:?}"),
            _ => {}
        }
    }
}