# Kubernetes Setup (Work In Progress)

<hr>

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


