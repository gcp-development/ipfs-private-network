# ipfs-private-network (Work In progress)

<h2>Motivation</h2>

The Web 3.0 library [lip2p](https://libp2p.io/) ([GO](https://github.com/libp2p/go-libp2p) and [Rust](https://github.com/libp2p/rust-libp2p) implementations are by far the most used), is a key modular set of networking tools for building peer-to-peer applications. Knowing what it does and when to used it, will save us of a lot of pain in our technical decisions and it will take care of our bank account. Web 3.0 represents a marked shift from a centralized ownership and control to a decentralized one and mistakes will be made. The important point is to never lose focus on the business model based in decentralized networks with creators, organizations, communities and users of content and data to their system with incentives, such as digital assets. That incentivize the users to become a part of the network and to participate by contributing to the network in some way and make "their living out of it".

Any technical solution or decisions should have this goal in mind and knowing your stuff helps in a long way to achieve this.

<hr>

<h2>Authentication</h2>

Every libp2p peer is uniquely identified by their [Peer ID](https://docs.libp2p.io/concepts/fundamentals/peers/#peer-id), which is derived from a private cryptographic key. Peer ids and their corresponding keys allow us to authenticate remote peers, so that we can be sure we’re talking to the correct peer and not an imposter.

<h2>How to handle authorization?</h2>

libp2p does not provide an authorization framework “out of the box”. To design an authorization system on libp2p, we need to rely on the authentication of peer ids and build an association between peer ids and permissions, with the Peer ID serving the same function as the “username” in traditional authorization frameworks, and the peer’s private key serving as the “password”. This allow us to reject requests from untrusted peers.

<h2>How to secure the channel ?</h2>

A [Noise Protocol](https://noiseprotocol.org/) begins with two parties exchanging handshake messages. During this handshake phase the parties exchange DH(Diffie-Hellman) public keys and perform a sequence of DH(Diffie-Hellman) operations, hashing the DH(Diffie-Hellman) results into a shared secret key. After the handshake phase each party can use this shared key to send encrypted transport messages.

<h2>Transport</h2>

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

<h2>Swarm struct</h2>

The [Swarm](https://docs.libp2p.io/concepts/appendix/glossary/#swarm) struct contains all active and pending connections to remotes and manages the state of all the substreams that have been opened, and all the upgrades that were built upon these substreams.

A Swarm requires three things:
<ul>
 <li>Identity of the local node <a href="https://docs.libp2p.io/concepts/appendix/glossary/#peerid" target="_blank">(PeerId)</a>.</li>
 <li>An implementation of the Transport <a href="https://doc.rust-lang.org/rust-by-example/trait.html" target="_blank">trait</a>.</li>
 <li>An implementation of the NetworkBehaviour <a href="https://doc.rust-lang.org/rust-by-example/trait.html" target="_blank">trait</a>.</li>
</ul>

<h2>Discovery mechanisms</h2>

The libp2p library enables many discovery mechanisms or even write our own. These are the two main ones: 

<ul>
  <li><a href="https://github.com/libp2p/specs/tree/master/rendezvous" target="_blank">Rendezvous Protocol</a>, lightweight mechanism for generalized peer discovery.</li>
 <li><a href="https://github.com/libp2p/specs/tree/master/identify" target="_blank">Identify</a> - <a href="https://github.com/libp2p/specs/tree/master/kad-dht" target="_blank">Kademlia</a>, Identify protocol using <a href="https://docs.libp2p.io/concepts/appendix/glossary/#peer-routing"  target="_blank">Peer Routing</a> (is the process of discovering peer addresses by using the knowledge of other peers) with DHT(Distributed Hash Table) implementation based on the Kademlia for storing those peer addresses.</li>
</ul>

Note: Mechanisms like bootstrap or [mDNS](https://github.com/libp2p/specs/blob/master/discovery/mdns.md) are limited. In the case of bootstrapping, only connect to the node listed and stops and in case of mDNS only connect to nodes found on the same LAN (local area network).

<a href="https://github.com/libp2p/specs/tree/master/autonat"  target="_blank">AutoNAT</a>

<hr>

Two Rust projects were developed to exemplify the concepts above described:
<ul>
<li><a href="https://github.com/gcp-development/ipfs-private-network/tree/main/basic-p2p" target="_blank">basic-p2p</a>, this is a simple project where we focus on a basic p2p application implementation using libp2p.</li>
<li><a href="https://github.com/gcp-development/ipfs-private-network/tree/main/libp2p-node" target="_blank">node-app</a>, this is a much complex project where we focus on the overlay network and the interaction with Identify protocol/DHT(Distributed Hash Table) Kademlia using libp2p.</li>
</ul>

<hr>

References:<br>
[Kademlia algorithm](https://docs.ipfs.tech/concepts/dht/#kademlia)<br>
[Central repository for work on libp2p](https://github.com/libp2p/rust-libp2p)<br>
[Security Considerations](https://docs.libp2p.io/concepts/security/security-considerations/)<br>
[Hole punching in libp2p - Overcoming Firewalls](https://blog.ipfs.tech/2022-01-20-libp2p-hole-punching/)<br>
