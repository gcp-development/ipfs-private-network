use libp2p::{
    core::{identity::Keypair, multiaddr::Multiaddr, upgrade::Version},
    noise,
    kad::{ record::store::MemoryStore, Kademlia, KademliaConfig, KademliaEvent},
    swarm::{NetworkBehaviour, SwarmBuilder, SwarmEvent},
    PeerId,
    Transport,
    tcp,
    yamux,
    identify
};

use std::{
    io::Read,
    fs::File,
    string::ToString,
    str::{FromStr},{error::Error},{time::Duration}
};

use futures::executor::block_on;
use futures::stream::StreamExt;

const PEER_ID_NODE_A:&str= "12D3KooWSMiGTt2roQoGDtUdwHtjivMi5sJJYNThTLHVD13eTqQ9";
const PEER_ID_MULTIADDR_NODE_A:&str= "/ip4/192.168.58.1/tcp/4201";

const PEER_ID_NODE_B:&str= "12D3KooWHh541fxK9mJsLxt8wX8cSCfzRsDrKTQaB8EG7R3RYj7z";
const PEER_ID_MULTIADDR_NODE_B:&str= "/ip4/192.168.58.1/tcp/4301";

const PEER_ID_NODE_C:&str= "12D3KooWJXMpHfCRtddGzZuN4z5Za3iAbikPt5Wav9vRUAxKzdEQ";
const PEER_ID_MULTIADDR_NODE_C:&str= "/ip4/192.168.58.1/tcp/4301";

const PEER_ID_NODE_D:&str= "12D3KooWSAj4PDGEUpywoe7FLcf6ancJmi3AEqACPwxDwZs3zW5g";
const PEER_ID_MULTIADDR_NODE_D:&str= "/ip4/192.168.58.1/tcp/4301";


#[derive(NetworkBehaviour)]
#[behaviour(out_event = "IdentifyAndKademliaEvent")]
struct IdentifyAndKademliaBehaviour {
    identify_behaviour: identify::Behaviour,
    kademlia_behaviour: Kademlia<MemoryStore>,
}

#[derive(Debug)]
#[allow(clippy::large_enum_variant)]
enum IdentifyAndKademliaEvent {
    Identify(identify::Event),
    Kademlia(KademliaEvent),
}

impl From<identify::Event> for IdentifyAndKademliaEvent {
    fn from(event: identify::Event) -> Self {
        IdentifyAndKademliaEvent::Identify(event)
    }
}

impl From<KademliaEvent> for IdentifyAndKademliaEvent {
    fn from(event: KademliaEvent) -> Self {
        IdentifyAndKademliaEvent::Kademlia(event)
    }
}

fn main()  -> Result<(), Box<dyn Error>> {

    let mut keypair_file_buffer = Vec::new();
    let keypair_file = File::open("keypair.bin");
    match keypair_file {
        Ok(mut file) => {
            file.read_to_end(&mut keypair_file_buffer).expect("keypair.bin");
        }
        Err(_) => {
            println!("Error reading file keypair.bin.");
        }
    }

    let mut local_key=Keypair::generate_ed25519();
    let result_keypair = Keypair::from_protobuf_encoding(&keypair_file_buffer);

    match result_keypair {
        Ok(keypair) => {
            local_key = keypair;
        }
        Err(_) => {
            println!("Not able to read the keypair file.");
        }
    }

    let mut peer_id_file_buffer = Vec::new();
    let read_file = File::open("peer_id.bin");
    match read_file {
        Ok(mut file) => {
            file.read_to_end(&mut peer_id_file_buffer).expect("peer_id.bin");
        }
        Err(_) => {
            println!("Error reading file peer_id.bin.");
        }
    }

    let mut local_peer_id = PeerId::from(local_key.public());
    let result_peer_id = PeerId::from_bytes(&peer_id_file_buffer);
    match result_peer_id {
        Ok(peer_id) => {
            local_peer_id = peer_id;
        }
        Err(_) => {
            println!("Not able to read the peer id file.");
        }
    }
    println!("PeerId: {:?}", local_peer_id);

    //Noise Keys
    let noise_keys = libp2p::noise::Keypair::<noise::X25519Spec>::new()
        .into_authentic(&local_key)
        .expect("Signing libp2p-noise static DH keypair failed.");

    // TCP (Standalone ⇄ Standalone) Transport encrypted with noise.
    let tcp_transport = tcp::async_io::Transport::new(tcp::Config::default())
        .upgrade(Version::V1)
        .authenticate(noise::NoiseConfig::xx(noise_keys).into_authenticated())
        .multiplex(yamux::Config::default())
        .timeout(Duration::from_secs(20))
        .boxed();

    let network_behaviour = {
        //Identify behaviour
        let identify_behaviour = identify::Behaviour::new(identify::Config::new("/ipfs/id/1.0.0".to_string(), local_key.public(), ));

        //Kademlia behaviour
        let mut kademlia_config = KademliaConfig::default();
        kademlia_config.set_query_timeout(Duration::from_secs(5 * 60));
        let memory_store = MemoryStore::new(local_peer_id);
        let mut kademlia_behaviour = Kademlia::with_config(local_peer_id, memory_store, kademlia_config);

        //Bootstrap nodes
        let peer_id_node_a = PeerId::from_str(PEER_ID_NODE_A).unwrap();
        let address_node_a = Multiaddr::from_str(PEER_ID_MULTIADDR_NODE_A).unwrap();
        kademlia_behaviour.add_address(&peer_id_node_a, address_node_a);

        let peer_id_node_b = PeerId::from_str(PEER_ID_NODE_B).unwrap();
        let address_node_b = Multiaddr::from_str(PEER_ID_MULTIADDR_NODE_B).unwrap();
        kademlia_behaviour.add_address(&peer_id_node_b, address_node_b);

        let peer_id_node_c = PeerId::from_str(PEER_ID_NODE_C).unwrap();
        let address_node_c = Multiaddr::from_str(PEER_ID_MULTIADDR_NODE_C).unwrap();
        kademlia_behaviour.add_address(&peer_id_node_c, address_node_c);

        let peer_id_node_d = PeerId::from_str(PEER_ID_NODE_D).unwrap();
        let address_node_d = Multiaddr::from_str(PEER_ID_MULTIADDR_NODE_D).unwrap();
        kademlia_behaviour.add_address(&peer_id_node_d, address_node_d);

        match kademlia_behaviour.bootstrap() {
            Ok(_) => {
                println!("Bootstrap node added.");
            }
            Err(_) => {
                println!("Bootstrap node not added.");
            }
        }

        IdentifyAndKademliaBehaviour {
            identify_behaviour,
            kademlia_behaviour,
        }
    };

    let mut swarm = { SwarmBuilder::without_executor(tcp_transport, network_behaviour, local_peer_id).build() };

    swarm.listen_on("/ip4/0.0.0.0/tcp/4001".parse().unwrap()).unwrap();

    block_on(async {
        loop {
            match swarm.next().await.expect("Infinite Stream.") {
                SwarmEvent::Behaviour(event) => {
                    println!("{event:?}")
                }
                SwarmEvent::NewListenAddr { address, .. } => {
                    println!("Listening on {address:?}");
                }
                _ => {}
            }
        }
    })
}