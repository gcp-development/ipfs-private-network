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

```bash
kubectl exec -it ipfs-node-a --namespace=overlay-network -- sh
```

```bash
ipfs id
```

![image](https://user-images.githubusercontent.com/76512851/232327266-9c5bea85-d3e4-4623-ab5a-75fe48f011b7.png)

Node-a:
<ul>
  <li>PeerId:12D3KooWNcb2eimZoc97x3ZV3ukQznHPxQXfqLP3Rci1WWRctMPC</li>
  <li>Address:/ip4/10.244.0.4/tcp/4001/p2p/12D3KooWNcb2eimZoc97x3ZV3ukQznHPxQXfqLP3Rci1WWRctMPC</li>
  <li>http://demo:30476/webui</li>
</ul>

Node-b:
<ul>
  <li>PeerId:12D3KooWQpyEz1PJ24GqPt9KxBewUaod9V4YFR8nvTCBQfRKPVVc</li>
  <li>Address:/ip4/10.244.0.5/tcp/4001/p2p/12D3KooWQpyEz1PJ24GqPt9KxBewUaod9V4YFR8nvTCBQfRKPVVc</li>
  <li>UI:http://demo:30476/webui</li>
</ul>

For the node-b:

```bash
kubectl exec -it ipfs-node-b --namespace=overlay-network -- sh
```

```bash
ipfs bootstrap rm all 
```

![image](https://user-images.githubusercontent.com/76512851/232328993-b655c567-36b8-4acd-838c-998d0730937c.png)

```bash
ipfs bootstrap add "/ip4/10.244.0.4/tcp/4001/p2p/12D3KooWNcb2eimZoc97x3ZV3ukQznHPxQXfqLP3Rci1WWRctMPC"
```

![image](https://user-images.githubusercontent.com/76512851/232329079-bbd20030-43d6-4875-a38e-0801998589dd.png)

```bash
ipfs swarm peers
```

![image](https://user-images.githubusercontent.com/76512851/232330410-65fa9b32-cc0c-4545-bb46-8d8c1ee26b85.png)

For the node-a:

```bash
kubectl exec -it ipfs-node-a --namespace=overlay-network -- sh
```

```bash
ipfs bootstrap rm all 
```

![image](https://user-images.githubusercontent.com/76512851/232328993-b655c567-36b8-4acd-838c-998d0730937c.png)

```bash
ipfs bootstrap add "/ip4/10.244.0.5/tcp/4001/p2p/12D3KooWQpyEz1PJ24GqPt9KxBewUaod9V4YFR8nvTCBQfRKPVVc"
```
![image](https://user-images.githubusercontent.com/76512851/232329420-082bb63f-ce08-4f4e-864b-f7298ccdea94.png)

```bash
ipfs swarm peers
```

![image](https://user-images.githubusercontent.com/76512851/232330338-968b2858-a046-45b7-91da-27c15de129fc.png)

Smoke Tests

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


