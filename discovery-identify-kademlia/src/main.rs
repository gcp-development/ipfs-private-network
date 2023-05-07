use libp2p::{
    core::{multiaddr::Multiaddr, upgrade::Version},
    kad::{record::store::MemoryStore, Kademlia, KademliaConfig, KademliaEvent},
    swarm::{NetworkBehaviour, SwarmBuilder, SwarmEvent},
    PeerId,
    Transport,
    tcp,
    yamux,
    identify,
    noise,
    kad
};

use std::{
    string::ToString,
    str::{FromStr},{error::Error},{time::Duration}
};
use std::task::Poll;
use log::{debug};
use futures::executor::block_on;
use futures::stream::StreamExt;
use futures_timer::Delay;
use libp2p::core::identity;

const PEER_ID_NODE_D:&str= "12D3KooWSAj4PDGEUpywoe7FLcf6ancJmi3AEqACPwxDwZs3zW5g";
const PEER_ID_MULTIADDR_NODE_D:&str= "/ip4/10.244.0.6/tcp/4001";

const BOOTSTRAP_INTERVAL: Duration = Duration::from_secs(5 * 60);

#[derive(NetworkBehaviour)]
#[behaviour(out_event = "IdentifyAndKademliaEvent", event_process = false)]
struct IdentifyAndKademliaBehaviour {
    identify_behaviour: identify::Behaviour,
    kademlia_behaviour: Kademlia<MemoryStore>,
}

#[derive(Debug)]
#[allow(clippy::large_enum_variant)]
enum IdentifyAndKademliaEvent {
    Identify(Box<identify::Event>),
    Kademlia(KademliaEvent),
}

impl From<identify::Event> for IdentifyAndKademliaEvent {
    fn from(event: identify::Event) -> Self {
        IdentifyAndKademliaEvent::Identify(Box::new(event))
    }
}

impl From<KademliaEvent> for IdentifyAndKademliaEvent {
    fn from(event: KademliaEvent) -> Self {
        IdentifyAndKademliaEvent::Kademlia(event)
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let local_key = identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());
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

        //node-d
        let peer_id_node_d = PeerId::from_str(PEER_ID_NODE_D).unwrap();
        let address_node_d = Multiaddr::from_str(PEER_ID_MULTIADDR_NODE_D).unwrap();
        kademlia_behaviour.add_address(&peer_id_node_d, address_node_d);

        match kademlia_behaviour.bootstrap() {
            Ok(_) => {
                println!("node-d added.");
            }
            Err(_) => {
                println!("node-d not added.");
            }
        }

        IdentifyAndKademliaBehaviour {
            identify_behaviour,
            kademlia_behaviour,
        }
    };

    let mut swarm = SwarmBuilder::with_async_std_executor(tcp_transport, network_behaviour, local_peer_id).build();
    swarm.listen_on("/ip4/0.0.0.0/tcp/4001".parse()?)?;

    let mut bootstrap_timer = Delay::new(BOOTSTRAP_INTERVAL);

    block_on(async {
        loop {
            if let Poll::Ready(()) = futures::poll!(&mut bootstrap_timer) {
                bootstrap_timer.reset(BOOTSTRAP_INTERVAL);
                let _ = swarm.behaviour_mut()
                    .kademlia_behaviour
                    .bootstrap();
            }

            match swarm.next().await.expect("Swarm not to terminate.") {
                SwarmEvent::Behaviour(IdentifyAndKademliaEvent::Identify(e)) => {
                    if let identify::Event::Received {
                        peer_id,
                        info:
                        identify::Info {
                            listen_addrs,
                            protocols,
                            ..
                        },
                    } = *e
                    {
                        if protocols
                            .iter()
                            .any(|p| p.as_bytes() == kad::protocol::DEFAULT_PROTO_NAME)
                        {
                            for addr in listen_addrs {
                                swarm.behaviour_mut()
                                    .kademlia_behaviour
                                    .add_address(&peer_id, addr.clone());
                                println!("Add node address");
                                println!("protocols {:?}", protocols);
                                println!("Address {}", addr);
                            }
                        }
                    }
                }
                SwarmEvent::Behaviour(IdentifyAndKademliaEvent::Kademlia(e)) => {
                    debug!("{:?}", e);
                }
                e => {
                    if let SwarmEvent::NewListenAddr { address, .. } = &e {
                        println!("Listening on {address:?}");
                    }
                }
            }
        }
    })
}
