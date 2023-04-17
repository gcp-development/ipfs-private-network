use libp2p::{identity::{Keypair},
             PeerId,
             swarm::{NetworkBehaviour,SwarmBuilder, SwarmEvent},
             gossipsub::{MessageId, ValidationMode, Event, IdentTopic, MessageAuthenticity},
             core::{transport::upgrade},
             kad::{record::{store::{MemoryStore}},Kademlia, KademliaEvent,KademliaConfig},
             Multiaddr,
             gossipsub,
             noise,
             Transport,
             mplex,
             tcp};

use std::{error::Error,
          time::Duration,
          str::FromStr
};

use cid::{Cid,
          multihash::{Code, MultihashDigest}};

use async_std::io;
use futures::{
    prelude::*,
    select
};


#[derive(NetworkBehaviour)]
#[behaviour(out_event = "KademliaAndGossipEvent")]
struct KademliaAndGossipBehaviour {
    kademlia_behaviour: Kademlia<MemoryStore>,
    gossipsub_behaviour: gossipsub::Behaviour,
}

#[allow(clippy::large_enum_variant)]
enum KademliaAndGossipEvent {
    Kademlia(KademliaEvent),
    Gossipsub(gossipsub::Event),
}

impl From<KademliaEvent> for KademliaAndGossipEvent {
    fn from(event: KademliaEvent) -> Self {
        KademliaAndGossipEvent::Kademlia(event)
    }
}

impl From<gossipsub::Event> for KademliaAndGossipEvent {
    fn from(event: gossipsub::Event) -> Self {
        KademliaAndGossipEvent::Gossipsub(event)
    }
}

#[async_std::main]
async fn main()  -> Result<(), Box<dyn Error>> {
    let local_key = Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());
    println!("Local peer id: {:?}", local_peer_id);

    //Noise Keys
    let noise_keys = noise::Keypair::<noise::X25519Spec>::new()
        .into_authentic(&local_key)
        .expect("Signing libp2p-noise static DH keypair failed.");

    // TCP (Standalone ⇄ Standalone) Transport encrypted with noise.
    let transport = tcp::async_io::Transport::new(tcp::Config::default())
        .upgrade(upgrade::Version::V1)
        .authenticate(noise::NoiseConfig::xx(noise_keys).into_authenticated())
        .multiplex(mplex::MplexConfig::new())
        .boxed();

    let network_behaviour = {
        // Message CID(Content identifier)
        let message_cid = |message: &gossipsub::Message| {
            let hash = Code::Sha2_256.digest(&message.data);
            let cid_v0 = Cid::new_v0(hash);
            MessageId::new(cid_v0.unwrap().clone().to_string().as_bytes())
        };

        //Kademlia behaviour
        let mut kademlia_config = KademliaConfig::default();
        kademlia_config.set_query_timeout(Duration::from_secs(5 * 60));
        let memory_store = MemoryStore::new(local_peer_id);
        let kademlia_behaviour = Kademlia::with_config(local_peer_id, memory_store, kademlia_config);

        // Gossip behaviour
        let gossipsub_config = gossipsub::ConfigBuilder::default()
            .heartbeat_interval(Duration::from_secs(10))
            .validation_mode(ValidationMode::Strict)
            .message_id_fn(message_cid)
            .build()
            .expect("Valid config");

        let gossipsub_behaviour: gossipsub::Behaviour = gossipsub::Behaviour::new(MessageAuthenticity::Signed(local_key), gossipsub_config)
            .expect("Failed creating an instance of the Gossip.");

        KademliaAndGossipBehaviour {
            kademlia_behaviour,
            gossipsub_behaviour,
        }
    };

    let mut swarm = { SwarmBuilder::without_executor(transport, network_behaviour, local_peer_id).build() };

    //Swarm
    swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse().unwrap()).unwrap();

    //Kademlia Behaviour  Bootstrap
    if let Some(peer_id) = std::env::args().nth(1) {
        if let Some(to_dial) = std::env::args().nth(2) {
            let multi_address_to_dial: Multiaddr = to_dial.parse().expect("Dial Address not valid.");
            swarm.behaviour_mut()
                .kademlia_behaviour
                .add_address(&PeerId::from_str(peer_id.as_str())?, multi_address_to_dial);
            swarm.behaviour_mut()
                .kademlia_behaviour
                .bootstrap()?;
        }
    }

    let topic = IdentTopic::new("node-topic");
    swarm.behaviour_mut().gossipsub_behaviour.subscribe(&topic)?;

    let mut stdin = io::BufReader::new(io::stdin()).lines().fuse();
    println!("Please enter a message.");

    loop {
        select! {
        line = stdin.select_next_some() => {
          if let Err(e) = swarm
            .behaviour_mut().gossipsub_behaviour
            .publish(topic.clone(), line.expect("Stdin not to close").as_bytes()) {
              println!("Publish error: {:?}", e);
          }
        },
        event = swarm.select_next_some() => match event {
          SwarmEvent::Behaviour(KademliaAndGossipEvent::Gossipsub(Event::Message {
              propagation_source: peer_id,
              message_id: id,
              message,
          })) =>  {
            println!(
              "Got message: '{}' with id: {id} from peer: {peer_id}",
              String::from_utf8_lossy(&message.data),
            );
          }
          SwarmEvent::NewListenAddr { address, .. } => {
              println!("Listening on {:?}", address);
          }
          _ => {}
        }
        }
    }
}