# Kubernetes Setup (Work In Progress)

<hr>

### Dockerfile

```bash
docker build -f /ipfs-private-network/kubernetes-setup/libp2p-private-network/node-a-dockerfile.dev -t node-a:1.0 .
```
Note : to list images just run "docker image ls"

```bash
docker tag node-a:1.0 {docker.hub}/node-a:1.0
```

```bash
docker push {docker.hub}/node-a:1.0
```


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

![image](https://user-images.githubusercontent.com/76512851/236633007-8db19213-71a2-45b7-9621-d48ddda6fede.png)

