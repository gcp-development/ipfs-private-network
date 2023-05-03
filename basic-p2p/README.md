# Basic p2p

This project was developed using the [Intellij Community](https://www.jetbrains.com/idea/download/#section=linux) with the [Rust plugin](https://www.jetbrains.com/rust/).

This is a simple project implementation using libp2p:
<ul>
  <li><a href="https://github.com/gcp-development/ipfs-private-network/blob/main/basic-p2p/src/main.rs#L66" target="_blank">Transport(TCP) encrypted with noise.</a></li>
  <li><a href="https://github.com/gcp-development/ipfs-private-network/blob/main/basic-p2p/src/main.rs#L72" target="_blank">Network Behaviour Kademlia and Gossip </a></li>
  <li><a href="https://github.com/gcp-development/ipfs-private-network/blob/main/basic-p2p/src/main.rs#L103" target="_blank">Swarm</a></li>
  <li><a href="https://github.com/gcp-development/ipfs-private-network/blob/main/basic-p2p/src/main.rs#L127" target="_blank">kickstarter loop</a></li>
</ul>

<hr>

## Test sending messages in two terminals in the same machine.

Open a new terminal(A) and execute:

```bash
cargo run
```
Terminal(A)

![image](https://user-images.githubusercontent.com/76512851/231508067-675a3bde-a299-4607-8cd9-41a79e70a27b.png)

Copy the peer id "12D3KooWD2tAnDrzjLc98niouLpaMbYBbEGuyqFeJ6RZBKNZ77dJ", the multiaddress "/ip4/127.0.0.1/tcp/39335" from the previous terminal(A) and open a another new terminal(B) and execute:

```bash
cargo run "12D3KooWD2tAnDrzjLc98niouLpaMbYBbEGuyqFeJ6RZBKNZ77dJ" "/ip4/127.0.0.1/tcp/39335"
```
Write a message "Hello" and press enter.

Terminal(B)

![image](https://user-images.githubusercontent.com/76512851/231510278-1d068411-a5d7-439e-8740-208a62a76f44.png)

Response from Terminal(A)
![image](https://user-images.githubusercontent.com/76512851/231511037-21026ce0-160a-40d8-99ab-cf4761f824f8.png)

Write a new message "bad cat" in the Terminal(B) and press enter.
Terminal(B)

![image](https://user-images.githubusercontent.com/76512851/231512254-8bca6cc3-98cb-423c-a46f-8998bb945c7e.png)

Response from Terminal(A)
![image](https://user-images.githubusercontent.com/76512851/231512422-b9d0eb3c-2eaa-4e47-8481-9cd64d37b83a.png)

## Test sending duplicade messages in two terminals in the same machine.


![image](https://user-images.githubusercontent.com/76512851/231515098-e5d32507-5bd2-46f4-b7bf-36cdfe021d09.png)

Open a new terminal(A) and execute:

```bash
cargo run
```
Terminal(A)

![image](https://user-images.githubusercontent.com/76512851/231515379-84bd950d-e57a-4281-a990-371724c3fe41.png)

Copy the peer id "12D3KooWKoEFcHuT7KFtQ8T8FbNAfyjecXjN97CYVnPMGFD9XUDh", the multiaddress "/ip4/127.0.0.1/tcp/34975" from the previous terminal(A) and open a another new terminal(B) and execute:

```bash
cargo run "12D3KooWKoEFcHuT7KFtQ8T8FbNAfyjecXjN97CYVnPMGFD9XUDh" "/ip4/127.0.0.1/tcp/34975"
```
Write a message "bella" and press enter.

![image](https://user-images.githubusercontent.com/76512851/231516640-71e8c52c-84b3-477a-ae35-66f864c99892.png)

Response from Terminal(A)

![image](https://user-images.githubusercontent.com/76512851/231516277-9b8440ad-9b80-46a1-879e-471183c2f1cd.png)

Write the same message "bella" in the Terminal(B) and press enter.

![image](https://user-images.githubusercontent.com/76512851/231516905-2bb79a3f-3919-4342-9fe1-771472004c94.png)

Write a new message "bad cat" in the Terminal(B) and press enter. Followed by a new message "bella" and press enter.

![image](https://user-images.githubusercontent.com/76512851/231517323-e460db33-0df6-4836-994e-d1dc49153d4e.png)

Response from Terminal(A)

![image](https://user-images.githubusercontent.com/76512851/231518093-6e7f9c3c-6094-4aea-b3ef-cb205223c332.png)

<hr>

References:<br>
[The Cargo Book](https://doc.rust-lang.org/cargo/)<br>
[gossipsub: An extensible baseline pubsub protocol](https://github.com/libp2p/specs/blob/master/pubsub/gossipsub/README.md)<br>
[libp2p Kademlia DHT specification](https://github.com/libp2p/specs/blob/master/kad-dht/README.md)<br>
