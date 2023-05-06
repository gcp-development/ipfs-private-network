# Kubernetes Setup (Work In Progress)


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

## Kubernetes manifests

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

Create a load balancer service for node-d.
```bash
kubectl apply -f 6_node-d-service.yml
```

![image](https://user-images.githubusercontent.com/76512851/236633007-8db19213-71a2-45b7-9621-d48ddda6fede.png)


![image](https://user-images.githubusercontent.com/76512851/236633865-c8003f54-1219-4f63-83a7-bf6a574d70ce.png)


![image](https://user-images.githubusercontent.com/76512851/236633883-eb0d697e-874e-4b1b-b157-5158da8c4513.png)

<hr>

References:<br>
[Kubernetes](https://kubernetes.io/docs/home/)<br>
