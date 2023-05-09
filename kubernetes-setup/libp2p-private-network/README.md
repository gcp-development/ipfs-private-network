# Kubernetes Setup


## Table of Contents<br>
- Dockerfile
- Kubernetes manifests
- Smoke tests 


<hr>

### Dockerfile

node-a (PeerId:12D3KooWAXY6cACWiab9uM4ss4Uas3Y6RwK5J3msFCvbMaZfcKaV)

Copy the files [keypair.bin and peer_id.bin](https://github.com/gcp-development/ipfs-private-network/tree/main/kubernetes-setup/libp2p-private-network/bootstrap/12D3KooWAXY6cACWiab9uM4ss4Uas3Y6RwK5J3msFCvbMaZfcKaV) into the project source.

![image](https://user-images.githubusercontent.com/76512851/236635561-1f97c777-1a35-4fbf-8dd0-8953b3c4fd82.png)

```bash
docker build -f /ipfs-private-network/kubernetes-setup/libp2p-private-network/node-a-dockerfile.dev -t node-a:1.0 .
```

```bash
docker tag node-a:1.0 {docker.hub}/node-a:1.0
```

```bash
docker push {docker.hub}/node-a:1.0
```

node-b (PeerId:12D3KooWHh541fxK9mJsLxt8wX8cSCfzRsDrKTQaB8EG7R3RYj7z)

Copy the files [keypair.bin and peer_id.bin](https://github.com/gcp-development/ipfs-private-network/tree/main/kubernetes-setup/libp2p-private-network/bootstrap/12D3KooWHh541fxK9mJsLxt8wX8cSCfzRsDrKTQaB8EG7R3RYj7z) into the project source.

```bash
docker build -f /ipfs-private-network/kubernetes-setup/libp2p-private-network/node-b-dockerfile.dev -t node-b:1.0 .
```

```bash
docker tag node-b:1.0 {docker.hub}/node-b:1.0
```

```bash
docker push {docker.hub}/node-b:1.0
```

node-c (PeerId:12D3KooWJXMpHfCRtddGzZuN4z5Za3iAbikPt5Wav9vRUAxKzdEQ)

Copy the files [keypair.bin and peer_id.bin](https://github.com/gcp-development/ipfs-private-network/tree/main/kubernetes-setup/libp2p-private-network/bootstrap/12D3KooWJXMpHfCRtddGzZuN4z5Za3iAbikPt5Wav9vRUAxKzdEQ) into the project source.

```bash
docker build -f /ipfs-private-network/kubernetes-setup/libp2p-private-network/node-c-dockerfile.dev -t node-c:1.0 .
```

```bash
docker tag node-c:1.0 {docker.hub}/node-c:1.0
```

```bash
docker push {docker.hub}/node-c:1.0
```

node-d (PeerId:12D3KooWSAj4PDGEUpywoe7FLcf6ancJmi3AEqACPwxDwZs3zW5g)

Copy the files [keypair.bin and peer_id.bin](https://github.com/gcp-development/ipfs-private-network/tree/main/kubernetes-setup/libp2p-private-network/bootstrap/12D3KooWSAj4PDGEUpywoe7FLcf6ancJmi3AEqACPwxDwZs3zW5g) into the project source.

```bash
docker build -f /ipfs-private-network/kubernetes-setup/libp2p-private-network/node-d-dockerfile.dev -t node-d:1.0 .
```

```bash
docker tag node-d:1.0 {docker.hub}/node-d:1.0
```

```bash
docker push {docker.hub}/node-d:1.0
```

Images created by the dockerfiles.

![image](https://user-images.githubusercontent.com/76512851/236635848-4de01a93-4cbf-4bb2-9a99-912cd3f1509f.png)

new-node

```bash
docker build -f /ipfs-private-network/kubernetes-setup/libp2p-private-network/node-d-dockerfile.dev -t new-node:1.0 .
```

```bash
docker tag new-node:1.0 {docker.hub}/new-node:1.0
```

```bash
docker push {docker.hub}/new-node:1.0
```

### Kubernetes manifests

Create a namespace.

```bash
kubectl apply -f 1_namespace.yml
```

Create a pod for node-a.
```bash
kubectl apply -f 2_node-a-pod.yml
```

Create a pod for node-b.
```bash
kubectl apply -f 3_node-b-pod.yml
```

Create a pod for node-c.
```bash
kubectl apply -f 4_node-c-pod.yml
```

Create a pod for node-d.
```bash
kubectl apply -f 5_node-d-pod.yml
```

![image](https://user-images.githubusercontent.com/76512851/236633007-8db19213-71a2-45b7-9621-d48ddda6fede.png)

Create a pod for new-node.
```bash
kubectl apply -f 6_new-node-pod.yml
```

### Smoke tests

Verify the logs from the new-node container.
```bash
kubectl logs -f new-node --namespace=overlay-network
```

![image](https://user-images.githubusercontent.com/76512851/236861260-6af6350f-a97e-4f58-870c-a8c046f64a21.png)

Verify the logs from the node-a container.
```bash
kubectl logs -f node-a --namespace=overlay-network
```

![image](https://user-images.githubusercontent.com/76512851/236863521-11d1d0f8-2dcb-4066-895b-a447b2202c20.png)

Verify the logs from the node-b container.
```bash
kubectl logs -f node-b --namespace=overlay-network
```

![image](https://user-images.githubusercontent.com/76512851/236863907-e24f434c-e034-4a75-8b3b-eab1d1a8a7a8.png)

Verify the logs from the node-c container.
```bash
kubectl logs -f node-c --namespace=overlay-network
```

![image](https://user-images.githubusercontent.com/76512851/236864082-57f0cdb1-74e5-4098-95c2-7903e9ede73e.png)

Verify the logs from the node-d container.
```bash
kubectl logs -f node-d --namespace=overlay-network
```
![image](https://user-images.githubusercontent.com/76512851/236864248-00724db7-60b7-46a2-91e5-38ac1c75d45d.png)

<hr>

References:<br>
[Kubernetes](https://kubernetes.io/docs/home/)<br>
