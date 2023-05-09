# ipfs-private-network (Work In progress)

<h2>Motivation</h2>

The Web 3.0 library [lip2p](https://libp2p.io/) ([GO](https://github.com/libp2p/go-libp2p) and [Rust](https://github.com/libp2p/rust-libp2p) implementations are by far the most used), is a key modular set of networking tools for building peer-to-peer applications. Knowing what it does and when to used it, will save us of a lot of pain in our technical decisions and it will take care of our bank account. Web 3.0 represents a marked shift from a centralized ownership and control to a decentralized one and mistakes will be made. The important point is to never lose focus on the business model based in decentralized networks with creators, organizations, communities and users of content and data to their system with incentives, such as digital assets. That incentivize the users to become a part of the network and to participate by contributing to the network in some way and make "their living out of it".

Any technical solution or decisions should have this goal in mind and knowing our stuff helps in a long way to achieve this.

<hr>

## Table of Contents<br>
- [IPFS vs libp2p](https://github.com/gcp-development/ipfs-private-network#ipfs-vs-libp2p)
- [libp2p](https://github.com/gcp-development/ipfs-private-network#libp2p)
  - [Authentication](https://github.com/gcp-development/ipfs-private-network#authentication)
  - [How to handle authorization?](https://github.com/gcp-development/ipfs-private-network#how-to-handle-authorization)
  - [How to secure the channel?](https://github.com/gcp-development/ipfs-private-network#how-to-secure-the-channel)
  - [Transport](https://github.com/gcp-development/ipfs-private-network#transport)
  - [Swarm struct](https://github.com/gcp-development/ipfs-private-network#swarm-struct)
  - [Discovery mechanisms](https://github.com/gcp-development/ipfs-private-network#discovery-mechanisms)
- [IPFS private network](https://github.com/gcp-development/ipfs-private-network#ipfs-private-network)
- [Identify+Kademlia discovery mechanism](https://github.com/gcp-development/ipfs-private-network#identifykademlia-discovery-mechanism)
- [Conclusion](https://github.com/gcp-development/ipfs-private-network#conclusion)

<hr>

## IPFS vs libp2p

Peer-to-peer networks have many advantages over the old client/server model, there are also challenges. In the process of overcoming these challenges, while building IPFS, the contributors of the IPFS project took care of building solutions in a modular, composable way, namely libp2p. Despite libp2p being the networking layer of IPFS, it does not require or depend on IPFS, and today many projects (like [Polkadot](https://polkadot.network/)) use libp2p as their network transport layer.

![image](https://user-images.githubusercontent.com/76512851/233556506-b375b779-53bf-4179-8e02-558ef4cd4cf2.png)

<hr>

## libp2p

### Authentication

Every libp2p peer is uniquely identified by their [Peer ID](https://docs.libp2p.io/concepts/fundamentals/peers/#peer-id), which is derived from a private cryptographic key. Peer ids and their corresponding keys allow us to authenticate remote peers, so that we can be sure we’re talking to the correct peer and not an imposter.

### How to handle authorization?</h2>

libp2p does not provide an authorization framework “out of the box”. To design an authorization system on libp2p, we need to rely on the authentication of peer ids and build an association between peer ids and permissions, with the Peer ID serving the same function as the “username” in traditional authorization frameworks, and the peer’s private key serving as the “password”. This allow us to reject requests from untrusted peers.

### How to secure the channel?</h2>

A [Noise Protocol](https://noiseprotocol.org/) begins with two parties exchanging handshake messages. During this handshake phase the parties exchange DH(Diffie-Hellman) public keys and perform a sequence of DH(Diffie-Hellman) operations, hashing the DH(Diffie-Hellman) results into a shared secret key. After the handshake phase each party can use this shared key to send encrypted transport messages.

### Transport

One of libp2p’s core requirements is to be transport agnostic. This means that the decision of what transport protocol to use is up to the us, and an application can support many different transports at the same time. 

<ul>
  <li>Standalone Node Connectivity</li>
<ul>
  <li><a href="https://connectivity.libp2p.io/#tcp" target="_blank">TCP (Standalone ⇄ Standalone)</a></li>
  <li><a href="https://connectivity.libp2p.io/#quic" target="_blank">QUIC-UDP (Standalone ⇄ Standalone)</a></li>
  <li><a href="https://connectivity.libp2p.io/#hole-punching" target="_blank">Hole Punching (Public Node ⇄ Private Node)</a></li>
</ul>
  <li>Browser Node Connectivity</li>
<ul>
  <li><a href="https://connectivity.libp2p.io/#websocket" target="_blank">WebSocket (Browser → Standalone)</a></li>
  <li><a href="https://connectivity.libp2p.io/#webtransport" target="_blank">WebTransport (Browser → Standalone)</a></li>
  <li><a href="https://connectivity.libp2p.io/#webrtc" target="_blank">WebRTC (Browser → Standalone) and (Browser ⇄ Browser)</a></li>
</ul>
</ul>

### Swarm struct

The [Swarm](https://docs.libp2p.io/concepts/appendix/glossary/#swarm) struct contains all active and pending connections to remotes and manages the state of all the substreams that have been opened, and all the upgrades that were built upon these substreams.

A Swarm requires three things:
<ul>
 <li>Identity of the local node <a href="https://docs.libp2p.io/concepts/appendix/glossary/#peerid" target="_blank">(PeerId)</a>.</li>
 <li>An implementation of the Transport <a href="https://doc.rust-lang.org/rust-by-example/trait.html" target="_blank">trait</a>.</li>
 <li>An implementation of the NetworkBehaviour <a href="https://doc.rust-lang.org/rust-by-example/trait.html" target="_blank">trait</a>.</li>
</ul>

### Discovery mechanisms

The libp2p library enables many discovery mechanisms or even write our own. These are the two main ones: 

<ul>
  <li><a href="https://github.com/libp2p/specs/tree/master/rendezvous" target="_blank">Rendezvous Protocol</a>, lightweight mechanism for generalized peer discovery.</li>
 <li><a href="https://github.com/libp2p/specs/tree/master/identify" target="_blank">Identify</a> - <a href="https://github.com/libp2p/specs/tree/master/kad-dht" target="_blank">Kademlia</a>, Identify protocol using <a href="https://docs.libp2p.io/concepts/appendix/glossary/#peer-routing"  target="_blank">Peer Routing</a> (is the process of discovering peer addresses by using the knowledge of other peers) with DHT(Distributed Hash Table) implementation based on the Kademlia for storing those peer addresses.</li>
</ul>

Note: Mechanisms like bootstrap or [mDNS](https://github.com/libp2p/specs/blob/master/discovery/mdns.md) are limited. In the case of bootstrapping, only connect to the node listed and stops and in case of mDNS only connect to nodes found on the same LAN (local area network).

A basic p2p application implementation using libp2p could be found in here [basic-p2p](https://github.com/gcp-development/ipfs-private-network/tree/main/basic-p2p).

<hr>

## IPFS private network

A [IPFS private network](https://github.com/gcp-development/ipfs-private-network/tree/main/kubernetes-setup) was created with two nodes. A private IPFS network allows only to connect to other peers who have a [shared secret key](https://github.com/gcp-development/ipfs-private-network/blob/main/kubernetes-setup/swarm.key). Each node will become part of the IPFS bootstrap list (is a list of peers with which the IPFS daemon learns about other peers on the network). Nodes in that network don't respond to communications from nodes outside that network.

![image](https://user-images.githubusercontent.com/76512851/233970800-41083b04-a616-4798-af22-28b36ebfdb39.png)

The Rust project [peer-identity](https://github.com/gcp-development/ipfs-private-network/tree/main/peer-identity) was used to retrieve the peer information using the Identify protocol.

Received info from peer Id 12D3KooWDfaWHmKi9XgrDw6e4tgu3noyjm8DmLRuQwyqVszdbcAe (node-a)

```bash
{
 public_key: Ed25519(PublicKey(compressed): eeb1fd8de179b6239867eb915496e6444feee9188fcaa7d9bf35f6dd7b2e20),
 protocol_version: "ipfs/0.1.0", agent_version: "kubo/0.21.0-dev/78895a1/docker",
 listen_addrs: ["/ip4/10.244.0.10/tcp/4001", "/ip4/10.244.0.10/udp/4001/quic", "/ip4/10.244.0.10/udp/4001/quic-v1"],
 protocols: ["/ipfs/ping/1.0.0", "/libp2p/circuit/relay/0.2.0/stop", "/ipfs/lan/kad/1.0.0", "/libp2p/autonat/1.0.0", "/ipfs/id/1.0.0", "/ipfs/id/push/1.0.0", "/ipfs/bitswap/1.2.0", "/ipfs/bitswap/1.1.0", "/ipfs/bitswap/1.0.0", "/ipfs/bitswap", "/x/"],
 observed_addr: "/ip4/10.244.0.1/tcp/45287"
}
```

Geting statistics about the node-a [DHT](https://docs.ipfs.tech/reference/kubo/cli/#ipfs-stats-dht)(s).
```bash
kubectl exec -it ipfs-node-a --namespace=overlay-network -- sh
ipfs stats dht
```
Protocols:
<ul>
  <li>DHT WAN:/ipfs/kad/1.0.0</li>
  <li>DHT LAN:/ipfs/lan/kad/1.0.0</li>
</ul>

![image](https://user-images.githubusercontent.com/76512851/234870754-76c322c1-31fa-409c-9c65-98e7b09619c8.png)


Received info from peer Id 12D3KooWRt8ciG9Bz2BpjKV3416fWcAvRPchGWXt6jNbaoTfJGto (node-b)

```bash
{
 public_key: Ed25519(PublicKey(compressed): 392e784b87befffcadaf4644fed53d215f7453d6da2a479acecd94af59b7ccb),
 protocol_version: "ipfs/0.1.0", agent_version: "kubo/0.21.0-dev/78895a1/docker",
 listen_addrs: ["/ip4/10.244.0.8/tcp/4001", "/ip4/10.244.0.8/udp/4001/quic", "/ip4/10.244.0.8/udp/4001/quic-v1"],
 protocols: ["/ipfs/ping/1.0.0", "/libp2p/circuit/relay/0.2.0/stop", "/ipfs/lan/kad/1.0.0", "/libp2p/autonat/1.0.0", "/ipfs/id/1.0.0", "/ipfs/id/push/1.0.0", "/ipfs/bitswap/1.2.0", "/ipfs/bitswap/1.1.0", "/ipfs/bitswap/1.0.0", "/ipfs/bitswap", "/x/"],
 observed_addr: "/ip4/10.244.0.1/tcp/5686"
}
```

Geting statistics about the node-b [DHT](https://docs.ipfs.tech/reference/kubo/cli/#ipfs-stats-dht)(s).
```bash
kubectl exec -it ipfs-node-b --namespace=overlay-network -- sh
ipfs stats dht
```
Protocols:
<ul>
  <li>DHT WAN:/ipfs/kad/1.0.0</li>
  <li>DHT LAN:/ipfs/lan/kad/1.0.0</li>
</ul>
  
![image](https://user-images.githubusercontent.com/76512851/234872994-09b9f4df-c514-4c22-bba7-4b49b5657992.png)

<hr>

## Identify+Kademlia discovery mechanism

The [Identify protocol](https://github.com/libp2p/specs/tree/master/identify) must be manually hooked up to [Kademlia](https://github.com/libp2p/specs/tree/master/kad-dht) through calls to [Kademlia::add_address](https://docs.rs/libp2p/0.51.3/libp2p/kad/struct.Kademlia.html#method.add_address). If we choose not to use the Identify protocol, and do not provide an alternative peer discovery mechanism, a Kademlia node will not discover nodes beyond the network's boot nodes. Without the Identify protocol, existing nodes in the kademlia network cannot obtain the listen addresses of nodes querying them, and thus will not be able to add them to their routing table.([Source](https://docs.rs/libp2p/0.51.3/libp2p/kad/index.html#important-discrepancies)).

| DHT  | Protocol | Discovery Mechanism |
| ------------- | ------------- | ------------- |
| WAN  | /ipfs/kad/1.0.0  | Identify protocol |
| LAN  | /ipfs/lan/kad/1.0.0  | Multicast DNS (mDNS) |

Note: This is called [Dual DHT](https://docs.ipfs.tech/concepts/dht/#dual-dht).

A [libp2p private network](https://github.com/gcp-development/ipfs-private-network/tree/main/kubernetes-setup/libp2p-private-network) was created with four nodes. Each [node](https://github.com/gcp-development/ipfs-private-network/tree/main/node) will have the [Identify](https://docs.rs/libp2p/0.51.3/libp2p/identify/index.html)("/ipfs/id/1.0.0") and [Kademlia](https://docs.rs/libp2p/0.51.3/libp2p/kad/index.html)("/ipfs/kad/1.0.0") behaviour configure. A new node [(discovery-Identify-kademlia)](https://github.com/gcp-development/ipfs-private-network/tree/main/discovery-Identify-kademlia) will be added to the libp2p private network and it will have to discover every other node in the network just by connecting to node-d using the Indentify+Kademlia behaviour.

![image](https://user-images.githubusercontent.com/76512851/236689105-fd01f2e4-2f8a-465e-b441-4570a2f7e83a.png)

Output from the [new-node](https://github.com/gcp-development/ipfs-private-network/tree/main/discovery-identify-kademlia)

```bash
PeerId: PeerId("12D3KooWLnyhf17V15nSPfJF77opdUbdGx2Q9zK7C48qucwuzgC9")
node-d added.
Listening on "/ip4/127.0.0.1/tcp/4001"
Listening on "/ip4/10.244.0.7/tcp/4001"
Add node
PeerId: PeerId("12D3KooWSAj4PDGEUpywoe7FLcf6ancJmi3AEqACPwxDwZs3zW5g")
Protocols: ["/ipfs/id/1.0.0", "/ipfs/id/push/1.0.0", "/ipfs/kad/1.0.0"]
Address /ip4/10.244.0.6/tcp/4001
Add node
PeerId: PeerId("12D3KooWSAj4PDGEUpywoe7FLcf6ancJmi3AEqACPwxDwZs3zW5g")
Protocols: ["/ipfs/id/1.0.0", "/ipfs/id/push/1.0.0", "/ipfs/kad/1.0.0"]
Address /ip4/10.244.0.6/tcp/4001
Add node
PeerId: PeerId("12D3KooWHh541fxK9mJsLxt8wX8cSCfzRsDrKTQaB8EG7R3RYj7z")
Protocols: ["/ipfs/id/1.0.0", "/ipfs/id/push/1.0.0", "/ipfs/kad/1.0.0"]
Address /ip4/10.244.0.4/tcp/4001
Add node
PeerId: PeerId("12D3KooWHh541fxK9mJsLxt8wX8cSCfzRsDrKTQaB8EG7R3RYj7z")
Protocols: ["/ipfs/id/1.0.0", "/ipfs/id/push/1.0.0", "/ipfs/kad/1.0.0"]
Address /ip4/10.244.0.4/tcp/4001
Add node
PeerId: PeerId("12D3KooWAXY6cACWiab9uM4ss4Uas3Y6RwK5J3msFCvbMaZfcKaV")
Protocols: ["/ipfs/id/1.0.0", "/ipfs/id/push/1.0.0", "/ipfs/kad/1.0.0"]
Address /ip4/10.244.0.3/tcp/4001
Add node
PeerId: PeerId("12D3KooWAXY6cACWiab9uM4ss4Uas3Y6RwK5J3msFCvbMaZfcKaV")
Protocols: ["/ipfs/id/1.0.0", "/ipfs/id/push/1.0.0", "/ipfs/kad/1.0.0"]
Address /ip4/10.244.0.3/tcp/4001
Add node
PeerId: PeerId("12D3KooWJXMpHfCRtddGzZuN4z5Za3iAbikPt5Wav9vRUAxKzdEQ")
Protocols: ["/ipfs/id/1.0.0", "/ipfs/id/push/1.0.0", "/ipfs/kad/1.0.0"]
Address /ip4/10.244.0.5/tcp/4001
Add node
PeerId: PeerId("12D3KooWJXMpHfCRtddGzZuN4z5Za3iAbikPt5Wav9vRUAxKzdEQ")
Protocols: ["/ipfs/id/1.0.0", "/ipfs/id/push/1.0.0", "/ipfs/kad/1.0.0"]
Address /ip4/10.244.0.5/tcp/4001
```

Note: The Rust project [create-keypair](https://github.com/gcp-development/ipfs-private-network/tree/main/create-keypair) will create the [identity keypair](https://docs.rs/libp2p/0.51.3/libp2p/core/identity/enum.Keypair.html), the corresponding [identifiers peers](https://docs.rs/libp2p/0.51.3/libp2p/struct.PeerId.html#) for the [four nodes](https://github.com/gcp-development/ipfs-private-network/tree/main/kubernetes-setup/libp2p-private-network/bootstrap) and save it in files(keypair.bin and peer_id.bin).

<hr>

## Conclusion

<hr>

References:<br>
[libp2p](https://docs.libp2p.io/)<br>
[Distributed Hash Tables (DHTs)](https://docs.ipfs.tech/concepts/dht)<br>
[Central repository for work on libp2p](https://github.com/libp2p/rust-libp2p)<br>
[Security Considerations](https://docs.libp2p.io/concepts/security/security-considerations/)<br>
