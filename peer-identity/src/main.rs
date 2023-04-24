use futures::prelude::*;
use libp2p::{
    core::{ multiaddr::Multiaddr, upgrade::Version},
    identify, identity, noise,
    swarm::{ SwarmBuilder, SwarmEvent},
    tcp,
    yamux,
    PeerId,
    Transport,
    pnet::{PnetConfig, PreSharedKey},
};
use std::{env ,fs ,time::Duration ,{error::Error},{string::ToString},path::Path ,str::{FromStr}};

const IPFS_NODE_A:&str= "/ip4/192.168.49.2/tcp/30882";
const IPFS_NODE_B:&str="/ip4/192.168.49.2/tcp/31524";

fn get_ipfs_path() -> Box<Path> {
    env::var("IPFS_PATH")
        .map(|ipfs_path| Path::new(&ipfs_path).into())
        .unwrap_or_else(|_| {
            env::var("HOME")
                .map(|home| Path::new(&home).join(".ipfs"))
                .expect("could not determine home directory")
                .into()
        })
}

fn get_psk(path: &Path) -> std::io::Result<Option<String>> {
    let swarm_key_file = path.join("swarm.key");
    match fs::read_to_string(swarm_key_file) {
        Ok(text) => Ok(Some(text)),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(e) => Err(e),
    }
}

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let key = "IPFS_PATH";
    env::set_var(key, "/home/silveira/ipfs/");

    let ipfs_path = get_ipfs_path();
    println!("using IPFS_PATH {ipfs_path:?}");
    let psk: Option<PreSharedKey> = get_psk(&ipfs_path)?
        .map(|text| PreSharedKey::from_str(&text))
        .transpose()?;

    let local_key = identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());
    println!("Local peer id: {local_peer_id:?}");

    if let Some(psk) = psk {
        println!("using swarm key with fingerprint: {}", psk.fingerprint());
    }

    let transport = tcp::async_io::Transport::default()
        .and_then(move |socket, _| PnetConfig::new(psk.unwrap()).handshake(socket))
        .upgrade(Version::V1)
        .authenticate(noise::NoiseAuthenticated::xx(&local_key).unwrap())
        .multiplex(yamux::YamuxConfig::default())
        .timeout(Duration::from_secs(20))
        .boxed();

    let behaviour = identify::Behaviour::new(identify::Config::new("/ipfs/id/1.0.0".to_string(), local_key.public(), ));

    let mut swarm = SwarmBuilder::with_async_std_executor(transport, behaviour, local_peer_id).build();

    swarm.listen_on("/ip4/0.0.0.0/tcp/44335".parse()?)?;

    let ipfs_node_a: Multiaddr = IPFS_NODE_A.parse()?;
    swarm.dial(ipfs_node_a)?;
    let ipfs_node_b: Multiaddr = IPFS_NODE_B.parse()?;
    swarm.dial(ipfs_node_b)?;

    loop {
        match swarm.select_next_some().await {
            SwarmEvent::Behaviour(identify::Event::Received { info, .. }) => {
                println!("Received {info:?}");
                println!("------------------------------------------------------------------------------------------------------------------------------------------------")
            }
            e => {
                if let SwarmEvent::NewListenAddr { address, .. } = &e {
                    println!("Listening on {:?}", address);
                    println!("------------------------------------------------------------------------------------------------------------------------------------------------");
                }
            }
        }
    }
}