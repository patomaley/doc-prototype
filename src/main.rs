use futures::prelude::*;
use libp2p::SwarmBuilder;
use libp2p::{identity, mdns, noise, ping, swarm::{Swarm, SwarmEvent}, tcp, yamux, core::upgrade::Version, Multiaddr, Transport};
use libp2p_swarm_derive::NetworkBehaviour;
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

#[derive(NetworkBehaviour)]
#[behaviour(out_event = "CombinedEvent")]
struct CombinedBehaviour {
    ping: ping::Behaviour,
    mdns: mdns::tokio::Behaviour,
}

#[derive(Debug)]
enum CombinedEvent {
    Ping(ping::Event),
    Mdns(mdns::Event),
}

impl From<ping::Event> for CombinedEvent {
    fn from(event: ping::Event) -> Self {
        CombinedEvent::Ping(event)
    }
}

impl From<mdns::Event> for CombinedEvent {
    fn from(event: mdns::Event) -> Self {
        CombinedEvent::Mdns(event)
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Storage test
    let note = Note {
        id: 1,
        title: "Test Note".to_string(),
        content: "Hello DOC - Physics efficient storage".to_string(),
    };
    save_note(&note)?;
    let loaded = load_note()?;
    println!("Loaded note: {:?}", loaded);

    // P2P with mDNS
    let local_key = identity::Keypair::generate_ed25519();
    let local_peer_id = local_key.public().to_peer_id();
    println!("Local peer id: {:?}", local_peer_id);

    let transport = tcp::tokio::Transport::default()
        .upgrade(Version::V1)
        .authenticate(noise::Config::new(&local_key)?)
        .multiplex(yamux::Config::default())
        .boxed();

    let behaviour = CombinedBehaviour {
        ping: ping::Behaviour::default(),
        mdns: mdns::tokio::Behaviour::new(mdns::Config::default(), local_peer_id)?,
    };

    let mut swarm = SwarmBuilder::with_existing_identity(local_key)
        .with_tokio()
        .with_other_transport(|_| Ok(transport))?
        .with_behaviour(|_| behaviour)?
        .build();

    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

    loop {
        match swarm.select_next_some().await {
            SwarmEvent::NewListenAddr { address, .. } => println!("Listening on {:?}", address),
            SwarmEvent::Behaviour(CombinedEvent::Ping(event)) => println!("Ping: {:?}", event),
            SwarmEvent::Behaviour(CombinedEvent::Mdns(mdns::Event::Discovered(list))) => {
                for (peer_id, addr) in list {
                    println!("Discovered {} at {}", peer_id, addr);
                    swarm.dial(addr.with(libp2p::multiaddr::Protocol::P2p(peer_id)))?;
                }
            }
            SwarmEvent::Behaviour(CombinedEvent::Mdns(mdns::Event::Expired(list))) => {
                for (peer_id, addr) in list {
                    println!("Expired {} at {}", peer_id, addr);
                }
            }
            _ => {}
        }
    }
}