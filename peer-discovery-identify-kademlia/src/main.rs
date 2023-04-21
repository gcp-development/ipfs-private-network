use futures::prelude::*;
use libp2p::{relay,core::{multiaddr::Multiaddr, upgrade::Version}, identify, identity, noise, swarm::{SwarmBuilder, SwarmEvent, NetworkBehaviour}, tcp, yamux, PeerId, Transport, kad::{store::MemoryStore, Kademlia, KademliaConfig, KademliaEvent}, ping::Event, ping, kad, autonat};
use std::{time::Duration, string::ToString, error::Error, thread};
use std::str::FromStr;
use std::task::Poll;
use futures::executor::block_on;
use libp2p::kad::{GetClosestPeersError, QueryResult};
use log::{debug, info};
use futures_timer::Delay;
use libp2p::metrics::{Metrics, Recorder};
use open_metrics_client::metrics::info::Info;
use open_metrics_client::registry::Registry;

#[derive(NetworkBehaviour)]
#[behaviour(out_event = "IdentifyAndKademliaEvent")]
struct IdentifyAndKademliaBehaviour {
    identify_behaviour: identify::Behaviour,
    kademlia_behaviour: Kademlia<MemoryStore>,
    ping_behaviour: ping::Behaviour,
    autonat_behaviour: autonat::Behaviour,
    relay_behaviour: relay::Behaviour,
}

#[allow(clippy::large_enum_variant)]
enum IdentifyAndKademliaEvent {
    Identify(Box<identify::Event>),
    Kademlia(KademliaEvent),
    Ping(libp2p::ping::Event),
    Autonat(autonat::Event),
    Relay(relay::Event),
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

impl From<libp2p::ping::Event> for IdentifyAndKademliaEvent {
    fn from(event: libp2p::ping::Event) -> Self {
        IdentifyAndKademliaEvent::Ping(event)
    }
}

impl From<autonat::Event> for IdentifyAndKademliaEvent {
    fn from(event: autonat::Event) -> Self {
        IdentifyAndKademliaEvent::Autonat(event)
    }
}

impl From<relay::Event> for IdentifyAndKademliaEvent {
    fn from(event: relay::Event) -> Self {
        IdentifyAndKademliaEvent::Relay(event)
    }
}

const BOOTSTRAP_INTERVAL: Duration = Duration::from_secs(5 * 60);
const IPFS_NODE_A:&str= "/ip4/192.168.49.2/tcp/31148";
const IPFS_NODE_B:&str= "/ip4/192.168.49.2/tcp/30965";

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

    let network_behaviour = {
        //Identify::Behaviour
        let identify_behaviour = identify::Behaviour::new(identify::Config::new("/ipfs/id/1.0.0".to_string(), local_key.public(), ));

        //Kademlia behaviour
        let mut kademlia_config = KademliaConfig::default();
        kademlia_config.set_query_timeout(Duration::from_secs(5 * 60));
        let memory_store = MemoryStore::new(local_peer_id);
        let mut kademlia_behaviour = Kademlia::with_config(local_peer_id, memory_store, kademlia_config);

        //Ping behaviour
        let ping_behaviour = libp2p::ping::Behaviour::new(libp2p::ping::Config::new());

        //Autonat behaviour
        let autonat_behaviour = autonat::Behaviour::new(PeerId::from(local_key.public().clone()), Default::default());

        //Relay behaviour
        let relay_behaviour = relay::Behaviour::new(PeerId::from(local_key.public().clone()), Default::default());

        IdentifyAndKademliaBehaviour {
            identify_behaviour,
            kademlia_behaviour,
            ping_behaviour,
            autonat_behaviour,
            relay_behaviour,
        }
    };

    let mut swarm = SwarmBuilder::with_async_std_executor(transport, network_behaviour, local_peer_id).build();

    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

    let ipfs_node_a: Multiaddr = IPFS_NODE_A.parse()?;
    swarm.dial(ipfs_node_a)?;
    //let ipfs_node_b: Multiaddr = IPFS_NODE_B.parse()?;
    //swarm.dial(ipfs_node_b)?;

    let mut bootstrap_timer = Delay::new(BOOTSTRAP_INTERVAL);

    loop {
        if let Poll::Ready(()) = futures::poll!(&mut bootstrap_timer) {
            bootstrap_timer.reset(BOOTSTRAP_INTERVAL);
            let _ = swarm
                .behaviour_mut().kademlia_behaviour.bootstrap();
        }
        match swarm.select_next_some().await {
            SwarmEvent::Behaviour(IdentifyAndKademliaEvent::Identify(e)) => {
                //println!("Sent identify info to {:?}", e);
                info!("{:?}", e);
                //metrics.record(&*e);
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
                    if protocols.iter().any(|p| p.as_bytes() == libp2p::kad::protocol::DEFAULT_PROTO_NAME)
                    {
                    for addr in listen_addrs {
                        println!("addr {:?}", addr);
                        swarm.behaviour_mut().kademlia_behaviour.add_address(&peer_id, addr);
                    }
                    println!("-----------------------------------------------------------------------------------------------------------------------------------------");
                    }
                }
            }
            SwarmEvent::Behaviour(IdentifyAndKademliaEvent::Autonat(e)) => {
                debug!("{:?}", e);
                //metrics.record(&e);
            }
            SwarmEvent::Behaviour(IdentifyAndKademliaEvent::Ping(e)) => {
                debug!("{:?}", e);
                //metrics.record(&e);
            }
            SwarmEvent::Behaviour(IdentifyAndKademliaEvent::Kademlia(e)) => {
                debug!("{:?}", e);
                //metrics.record(&e);
            }
            e => {
                if let SwarmEvent::NewListenAddr { address, .. } = &e {
                    println!("Listening on {:?}", address);
                }

                //metrics.record(&e)
            }
        }
    }
}
