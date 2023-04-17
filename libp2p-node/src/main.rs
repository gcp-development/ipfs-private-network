use futures::StreamExt;
use libp2p::kad::record::store::MemoryStore;
use libp2p::kad::{GetClosestPeersError, Kademlia, KademliaConfig, KademliaEvent, QueryResult};
use libp2p::{
    development_transport, identity,
    swarm::{SwarmBuilder, SwarmEvent},
    PeerId,
    Multiaddr,
};
use std::{env, error::Error, time::Duration};
use std::str::FromStr;

const BOOTNODES: [&str; 1] = [
    "12D3KooWQpyEz1PJ24GqPt9KxBewUaod9V4YFR8nvTCBQfRKPVVc",
];

#[async_std::main]
async fn main() -> Result<(), Box<dyn Error>> {
    env_logger::init();

    // Create a random key for ourselves.
    let local_key = identity::Keypair::generate_ed25519();
    let local_peer_id = PeerId::from(local_key.public());

    // Set up a an encrypted DNS-enabled TCP Transport over the Mplex protocol
    let transport = development_transport(local_key).await?;

    // Create a swarm to manage peers and events.
    let mut swarm = {
        // Create a Kademlia behaviour.
        let mut cfg = KademliaConfig::default();
        cfg.set_query_timeout(Duration::from_secs(5 * 60));
        let store = MemoryStore::new(local_peer_id);
        let mut behaviour = Kademlia::with_config(local_peer_id, store, cfg);

        // Add the bootnodes to the local routing table. `libp2p-dns` built
        // into the `transport` resolves the `dnsaddr` when Kademlia tries
        // to dial these nodes.
        ///ip4/demo/tcp/31148/p2p/12D3KooWQpyEz1PJ24GqPt9KxBewUaod9V4YFR8nvTCBQfRKPVVc
        let bootaddr = Multiaddr::from_str("/ip4/192.168.49.2/tcp/30965/p2p/12D3KooWQpyEz1PJ24GqPt9KxBewUaod9V4YFR8nvTCBQfRKPVVc")?;
        for peer in &BOOTNODES {
            behaviour.add_address(&PeerId::from_str(peer)?, bootaddr.clone());
            //behaviour.add_address(&PeerId::from_str(peer)?,  "/dnsaddr/bootstrap.libp2p.io".parse()?);
        }

        //for peer in &BOOTNODES {
        //    behaviour.add_address(&peer.parse()?, "/dnsaddr/bootstrap.libp2p.io".parse()?);
       // }

        SwarmBuilder::with_async_std_executor(transport, behaviour, local_peer_id).build()
    };

    // Order Kademlia to search for a peer.
    let to_search = env::args()
        .nth(1)
        .map(|p| p.parse())
        .transpose()?
        .unwrap_or_else(PeerId::random);

    println!("Searching for the closest peers to {to_search}");
    swarm.behaviour_mut().get_closest_peers(to_search);

    loop {
        let event = swarm.select_next_some().await;
        if let SwarmEvent::Behaviour(KademliaEvent::OutboundQueryProgressed {
                                         result: QueryResult::GetClosestPeers(result),
                                         ..
                                     }) = event
        {
            match result {
                Ok(ok) => {
                    if !ok.peers.is_empty() {
                        println!("Query finished with closest peers: {:#?}", ok.peers)
                    } else {
                        // The example is considered failed as there
                        // should always be at least 1 reachable peer.
                        println!("Query finished with no closest peers.")
                    }
                }
                Err(GetClosestPeersError::Timeout { peers, .. }) => {
                    if !peers.is_empty() {
                        println!("Query timed out with closest peers: {peers:#?}")
                    } else {
                        // The example is considered failed as there
                        // should always be at least 1 reachable peer.
                        println!("Query timed out with no closest peers.");
                    }
                }
            };

            //break;
        }
    }

    //Ok(())
}