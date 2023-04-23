# Kubernetes Setup (Work In Progress)
It's assumed that these software are installed and running:

<ul>
  <li><a href="https://docs.docker.com/engine/install/ubuntu/" target="_blank">docker</a></li>
  <li><a href="https://minikube.sigs.k8s.io/docs/start/" target="_blank">minikube</a></li>
  <li><a href="https://kubernetes.io/docs/tasks/tools/install-kubectl-linux/" target="_blank">kubectl</a></li>
</ul>
<hr>

## minikube setup

minikube version

![image](https://user-images.githubusercontent.com/76512851/222912565-1742b8a7-2b23-45f2-9007-bb1ade990be1.png)

Create a [cluster](https://minikube.sigs.k8s.io/docs/commands/profile/).
```bash
minikube start --driver=docker -p demo
```
![image](https://user-images.githubusercontent.com/76512851/222913292-c33b7a20-b00f-49f8-a8df-3bca70837d51.png)

<hr>

```bash
kubectl apply -f 1_namespace.yml
```

```bash
kubectl apply -f 2_ipfs-node-a-pv.yml
```

```bash
kubectl apply -f 3_ipfs-node-a-pvc.yml
```

```bash
kubectl apply -f 4_ipfs-node-a-pod.yml
```

```bash
kubectl apply -f 5_ipfs-node-a-service.yml
```

```bash
kubectl apply -f 6_ipfs-node-b-pv.yml

```

```bash
kubectl apply -f 7_ipfs-node-b-pvc.yml

```

```bash
kubectl apply -f 8_ipfs-node-b-pod.yml
```

```bash
kubectl apply -f 9_ipfs-node-b-service.yml
```

```bash
minikube tunnel -p demo
```

```bash
kubectl get svc --namespace=overlay-network
```

![image](https://user-images.githubusercontent.com/76512851/232242406-bc796349-c4e3-44d4-a760-6978c68be56a.png)

```bash
kubectl exec -it ipfs-node-a --namespace=overlay-network -- sh
```

```bash
ipfs config --json API.HTTPHeaders.Access-Control-Allow-Origin '["http://demo:30385", "http://localhost:3000", "http://127.0.0.1:5001", "https://webui.ipfs.io"]'
ipfs config --json API.HTTPHeaders.Access-Control-Allow-Methods '["PUT", "POST"]'
```

```bash
ipfs shutdown
```

Create a swarm key.

```bash
echo -e "/key/swarm/psk/1.0.0/\n/base16/\n`tr -dc 'a-f0-9' < /dev/urandom | head -c64`" > swarm.key
```

Copy the swarm key to both kubo nodes into the "./ipfs" directory.

![image](https://user-images.githubusercontent.com/76512851/232326721-590c47ed-9b17-4190-abe2-d018a644b1ba.png)


Bootstrap the two IPFS nodes.

<hr>

## node-a

Login into the node-a.

```bash
kubectl exec -it ipfs-node-a --namespace=overlay-network -- sh
```
 
 Get the IPFS node-a id info.
 
```bash
ipfs id
```

```bash
{
	"ID": "12D3KooWDfaWHmKi9XgrDw6e4tgu3noyjm8DmLRuQwyqVszdbcAe",
	"PublicKey": "CAESIDkueEuHvv/8ra9GRP7VPSFfdFMNbaKkeazs2Ur1m3zL",
	"Addresses": [
		"/ip4/10.244.0.6/tcp/4001/p2p/12D3KooWDfaWHmKi9XgrDw6e4tgu3noyjm8DmLRuQwyqVszdbcAe",
		"/ip4/10.244.0.6/udp/4001/quic-v1/p2p/12D3KooWDfaWHmKi9XgrDw6e4tgu3noyjm8DmLRuQwyqVszdbcAe",
		"/ip4/10.244.0.6/udp/4001/quic-v1/webtransport/certhash/uEiBbxxmTGYfq5yfJlfEhVqZw3jaXvwHkUwTbCextejV_vQ/certhash/uEiDuSoxQYdkKvPKSp32-8jFemuC8F6vsBTKjVYddm_trmg/p2p/12D3KooWDfaWHmKi9XgrDw6e4tgu3noyjm8DmLRuQwyqVszdbcAe",
		"/ip4/10.244.0.6/udp/4001/quic/p2p/12D3KooWDfaWHmKi9XgrDw6e4tgu3noyjm8DmLRuQwyqVszdbcAe",
		"/ip4/127.0.0.1/tcp/4001/p2p/12D3KooWDfaWHmKi9XgrDw6e4tgu3noyjm8DmLRuQwyqVszdbcAe",
		"/ip4/127.0.0.1/udp/4001/quic-v1/p2p/12D3KooWDfaWHmKi9XgrDw6e4tgu3noyjm8DmLRuQwyqVszdbcAe",
		"/ip4/127.0.0.1/udp/4001/quic-v1/webtransport/certhash/uEiBbxxmTGYfq5yfJlfEhVqZw3jaXvwHkUwTbCextejV_vQ/certhash/uEiDuSoxQYdkKvPKSp32-8jFemuC8F6vsBTKjVYddm_trmg/p2p/12D3KooWDfaWHmKi9XgrDw6e4tgu3noyjm8DmLRuQwyqVszdbcAe",
		"/ip4/127.0.0.1/udp/4001/quic/p2p/12D3KooWDfaWHmKi9XgrDw6e4tgu3noyjm8DmLRuQwyqVszdbcAe",
		"/ip4/154.53.46.59/tcp/4001/p2p/12D3KooWD6jzDPEFDN8pjozFs2HVzvBfcsQYgzmrAi77rJfk9ghA/p2p-circuit/p2p/12D3KooWDfaWHmKi9XgrDw6e4tgu3noyjm8DmLRuQwyqVszdbcAe",
		"/ip4/154.53.46.59/udp/4001/quic-v1/p2p/12D3KooWD6jzDPEFDN8pjozFs2HVzvBfcsQYgzmrAi77rJfk9ghA/p2p-circuit/p2p/12D3KooWDfaWHmKi9XgrDw6e4tgu3noyjm8DmLRuQwyqVszdbcAe",
		"/ip4/154.53.46.59/udp/4001/quic/p2p/12D3KooWD6jzDPEFDN8pjozFs2HVzvBfcsQYgzmrAi77rJfk9ghA/p2p-circuit/p2p/12D3KooWDfaWHmKi9XgrDw6e4tgu3noyjm8DmLRuQwyqVszdbcAe",
		"/ip4/66.42.107.0/tcp/4001/p2p/12D3KooWNFrxvqzQDrVWfVFfTezfakB3FAJJRw8kKhRNcw8MGUQE/p2p-circuit/p2p/12D3KooWDfaWHmKi9XgrDw6e4tgu3noyjm8DmLRuQwyqVszdbcAe",
		"/ip4/66.42.107.0/udp/4001/quic-v1/p2p/12D3KooWNFrxvqzQDrVWfVFfTezfakB3FAJJRw8kKhRNcw8MGUQE/p2p-circuit/p2p/12D3KooWDfaWHmKi9XgrDw6e4tgu3noyjm8DmLRuQwyqVszdbcAe",
		"/ip4/66.42.107.0/udp/4001/quic/p2p/12D3KooWNFrxvqzQDrVWfVFfTezfakB3FAJJRw8kKhRNcw8MGUQE/p2p-circuit/p2p/12D3KooWDfaWHmKi9XgrDw6e4tgu3noyjm8DmLRuQwyqVszdbcAe"
	],
	"AgentVersion": "kubo/0.21.0-dev/78895a1/docker",
	"ProtocolVersion": "ipfs/0.1.0",
	"Protocols": [
		"/ipfs/bitswap",
		"/ipfs/bitswap/1.0.0",
		"/ipfs/bitswap/1.1.0",
		"/ipfs/bitswap/1.2.0",
		"/ipfs/id/1.0.0",
		"/ipfs/id/push/1.0.0",
		"/ipfs/lan/kad/1.0.0",
		"/ipfs/ping/1.0.0",
		"/libp2p/circuit/relay/0.2.0/stop",
		"/libp2p/dcutr",
		"/x/"
	]
}
```

Remove the peers from the node-a bootstrap list.

```bash
ipfs bootstrap rm all 
```

![image](https://user-images.githubusercontent.com/76512851/232328993-b655c567-36b8-4acd-838c-998d0730937c.png)

Add the node-b to the node-a bootstrap list.

```bash
ipfs bootstrap add "/ip4/10.244.0.7/tcp/4001/p2p/12D3KooWRt8ciG9Bz2BpjKV3416fWcAvRPchGWXt6jNbaoTfJGto"
```

Get the peers from the node-a bootstrap list.

```bash
ipfs bootstrap list
```

![image](https://user-images.githubusercontent.com/76512851/233839859-7a21d069-02e2-4776-855e-f6a642c64e03.png)


<hr>

## node-b

Login into the node-b.

```bash
kubectl exec -it ipfs-node-b --namespace=overlay-network -- sh
```

Get the IPFS node-b id info.
 
```bash
ipfs id
```

```bash
{
	"ID": "12D3KooWRt8ciG9Bz2BpjKV3416fWcAvRPchGWXt6jNbaoTfJGto",
	"PublicKey": "CAESIO6x/Y3hebYjmGfrkVSWDgZET+7pGI/Kp9m/Nfbdey4g",
	"Addresses": [
		"/ip4/10.244.0.7/tcp/4001/p2p/12D3KooWRt8ciG9Bz2BpjKV3416fWcAvRPchGWXt6jNbaoTfJGto",
		"/ip4/10.244.0.7/udp/4001/quic-v1/p2p/12D3KooWRt8ciG9Bz2BpjKV3416fWcAvRPchGWXt6jNbaoTfJGto",
		"/ip4/10.244.0.7/udp/4001/quic-v1/webtransport/certhash/uEiDxtDg_kh_UDtk3-ZP6-vJtTOXWsBsFLbVFzDyDBAkYLw/certhash/uEiCpMr9FUtxtMMNllO-yKksgfK-eHC75Fye9LmkCGPFzLQ/p2p/12D3KooWRt8ciG9Bz2BpjKV3416fWcAvRPchGWXt6jNbaoTfJGto",
		"/ip4/10.244.0.7/udp/4001/quic/p2p/12D3KooWRt8ciG9Bz2BpjKV3416fWcAvRPchGWXt6jNbaoTfJGto",
		"/ip4/127.0.0.1/tcp/4001/p2p/12D3KooWRt8ciG9Bz2BpjKV3416fWcAvRPchGWXt6jNbaoTfJGto",
		"/ip4/127.0.0.1/udp/4001/quic-v1/p2p/12D3KooWRt8ciG9Bz2BpjKV3416fWcAvRPchGWXt6jNbaoTfJGto",
		"/ip4/127.0.0.1/udp/4001/quic-v1/webtransport/certhash/uEiDxtDg_kh_UDtk3-ZP6-vJtTOXWsBsFLbVFzDyDBAkYLw/certhash/uEiCpMr9FUtxtMMNllO-yKksgfK-eHC75Fye9LmkCGPFzLQ/p2p/12D3KooWRt8ciG9Bz2BpjKV3416fWcAvRPchGWXt6jNbaoTfJGto",
		"/ip4/127.0.0.1/udp/4001/quic/p2p/12D3KooWRt8ciG9Bz2BpjKV3416fWcAvRPchGWXt6jNbaoTfJGto",
		"/ip4/155.138.212.178/tcp/4001/p2p/12D3KooWRje7cgvGXUDRe2zwBBrSdFJyT9SAZVqyLAmTAfCqjrcp/p2p-circuit/p2p/12D3KooWRt8ciG9Bz2BpjKV3416fWcAvRPchGWXt6jNbaoTfJGto",
		"/ip4/155.138.212.178/udp/4001/quic-v1/p2p/12D3KooWRje7cgvGXUDRe2zwBBrSdFJyT9SAZVqyLAmTAfCqjrcp/p2p-circuit/p2p/12D3KooWRt8ciG9Bz2BpjKV3416fWcAvRPchGWXt6jNbaoTfJGto",
		"/ip4/155.138.212.178/udp/4001/quic/p2p/12D3KooWRje7cgvGXUDRe2zwBBrSdFJyT9SAZVqyLAmTAfCqjrcp/p2p-circuit/p2p/12D3KooWRt8ciG9Bz2BpjKV3416fWcAvRPchGWXt6jNbaoTfJGto",
		"/ip4/185.239.209.221/tcp/4001/p2p/12D3KooWQN8VkacB3e1521CBUnEukEQRr96Cb7qpvqKK1n5zmZLd/p2p-circuit/p2p/12D3KooWRt8ciG9Bz2BpjKV3416fWcAvRPchGWXt6jNbaoTfJGto",
		"/ip4/185.239.209.221/udp/4001/quic-v1/p2p/12D3KooWQN8VkacB3e1521CBUnEukEQRr96Cb7qpvqKK1n5zmZLd/p2p-circuit/p2p/12D3KooWRt8ciG9Bz2BpjKV3416fWcAvRPchGWXt6jNbaoTfJGto",
		"/ip4/185.239.209.221/udp/4001/quic/p2p/12D3KooWQN8VkacB3e1521CBUnEukEQRr96Cb7qpvqKK1n5zmZLd/p2p-circuit/p2p/12D3KooWRt8ciG9Bz2BpjKV3416fWcAvRPchGWXt6jNbaoTfJGto"
	],
	"AgentVersion": "kubo/0.21.0-dev/78895a1/docker",
	"ProtocolVersion": "ipfs/0.1.0",
	"Protocols": [
		"/ipfs/bitswap",
		"/ipfs/bitswap/1.0.0",
		"/ipfs/bitswap/1.1.0",
		"/ipfs/bitswap/1.2.0",
		"/ipfs/id/1.0.0",
		"/ipfs/id/push/1.0.0",
		"/ipfs/lan/kad/1.0.0",
		"/ipfs/ping/1.0.0",
		"/libp2p/circuit/relay/0.2.0/stop",
		"/libp2p/dcutr",
		"/x/"
	]
}

```

Remove the peers from the node-b bootstrap list.

```bash
ipfs bootstrap rm all
```

![image](https://user-images.githubusercontent.com/76512851/232328993-b655c567-36b8-4acd-838c-998d0730937c.png)

Add the node-a to the node-b bootstrap list.

```bash
ipfs bootstrap add "/ip4/10.244.0.6/tcp/4001/p2p/12D3KooWDfaWHmKi9XgrDw6e4tgu3noyjm8DmLRuQwyqVszdbcAe"
```

Get the peers from the node-b bootstrap list.

```bash
ipfs bootstrap list
```
![image](https://user-images.githubusercontent.com/76512851/233838668-a28d88a1-e5cf-4fe1-b7ea-0629c0a0197e.png)

## Smoke Tests

### WebUI node-a

![image](https://user-images.githubusercontent.com/76512851/233844139-09f414d7-c44a-413d-aada-3c2bd2351e14.png)

### WebUI node-b

![image](https://user-images.githubusercontent.com/76512851/233844184-ac6b5630-0d7d-4319-88df-586ff6ba7284.png)

### Swarm (node-a)

Login into the node-a.

```bash
kubectl exec -it ipfs-node-a --namespace=overlay-network -- sh
```

List peers with open connections in  node-a.

```bash
ipfs swarm peers
```

![image](https://user-images.githubusercontent.com/76512851/233841177-f8137248-0b92-43db-9ab2-1afdea3005f4.png)

WebUI list of known peers.
![image](https://user-images.githubusercontent.com/76512851/233841100-fda48249-206f-422e-b1a9-4d9416d30b8f.png)

### Swarm (node-b)

Login into the node-b.

```bash
kubectl exec -it ipfs-node-b --namespace=overlay-network -- sh
```

List peers with open connections in  node-b.

```bash
ipfs swarm peers```
```

![image](https://user-images.githubusercontent.com/76512851/233840535-19f6fb2a-3cf9-4000-9705-585c2893ae23.png)

WebUI list of known peers.

![image](https://user-images.githubusercontent.com/76512851/233840645-fe94134a-b941-4d90-aee9-592c96260842.png)

### Get file from Swarm peers

In the node-b, add a file.

```bash
echo hello from node b > nodeb_file.txt
ipfs add nodeb_file.txt
```

![image](https://user-images.githubusercontent.com/76512851/232331654-1062808b-f1d1-401a-9a7f-159eafbd44a3.png)

In the node-a, open the file added in node-b.

```bash
ipfs cat QmNVcMVkMSo8yvPfJJXU8nMqpu5U7oEzBnWbirKx3zprH8
```

![image](https://user-images.githubusercontent.com/76512851/232331785-5dac16cd-3152-4837-94bc-5dee8cb2985e.png)

<hr>

References:<br>
[Kubernetes](https://kubernetes.io/docs/home/)<br>
[IPFS](https://docs.ipfs.tech/)<br>
