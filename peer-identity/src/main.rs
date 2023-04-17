use futures::prelude::*;
use libp2p::{
    core::{multiaddr::Multiaddr, upgrade::Version},
    identify, identity, noise,
    swarm::{SwarmBuilder, SwarmEvent},
    tcp, yamux, PeerId, Transport,
};
use std::error::Error;
use std::string::ToString;

const IPFS_NODE_A:&str= "/ip4/192.168.49.2/tcp/31148";
const IPFS_NODE_B:&str="/ip4/192.168.49.2/tcp/30965";

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let local_key = identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());
    println!("Local peer id: {local_peer_id:?}");

    let transport = tcp::async_io::Transport::default()
        .upgrade(Version::V1)
        .authenticate(noise::NoiseAuthenticated::xx(&local_key).unwrap())
        .multiplex(yamux::YamuxConfig::default())
        .boxed();

    let behaviour = identify::Behaviour::new(identify::Config::new("/ipfs/id/1.0.0".to_string(), local_key.public(), ));

    let mut swarm = SwarmBuilder::with_async_std_executor(transport, behaviour, local_peer_id).build();

    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

    let ipfs_node_a: Multiaddr = IPFS_NODE_A.parse()?;
    swarm.dial(ipfs_node_a)?;
    let ipfs_node_b: Multiaddr = IPFS_NODE_B.parse()?;
    swarm.dial(ipfs_node_b)?;

    loop {
        match swarm.select_next_some().await {
            SwarmEvent::Behaviour(identify::Event::Sent { peer_id, .. }) => {
                println!("Sent identify info to {peer_id:?}");
                println!("------------------------------------------------------------------------------------------------------------------------------------------------")
            }
            SwarmEvent::Behaviour(identify::Event::Received { info, .. }) => {
                println!("Received {info:?}");
                println!("------------------------------------------------------------------------------------------------------------------------------------------------")
            }
            _ => {}
        }
    }
}
