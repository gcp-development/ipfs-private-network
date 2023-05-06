
# node

This project was developed using the [Intellij Community](https://www.jetbrains.com/idea/download/#section=linux) with the [Rust plugin](https://www.jetbrains.com/rust/).

To compile in release mode.

```bash
cargo build --release
```

To run the application.

```bash
cargo run <pod>
```
Note: <pod> - a,b,c or d.

Remove al artifacts from the target directory generated in the past.

```bash
cargo clean
```

Where the pods IPs are defined.

![image](https://user-images.githubusercontent.com/76512851/236634324-893166a5-d53d-464f-9af4-2f774538bc50.png)

How do we know the Pods IPs?

```bash
kubectl get po --all-namespaces -o wide
```

![image](https://user-images.githubusercontent.com/76512851/236634595-33fb25ce-5503-4910-bd64-44ef2320eae4.png)

The kube-system (coredns-787d4945fb-nqllx) is responsible to create [DNS records](https://kubernetes.io/docs/concepts/services-networking/dns-pod-service/) for [Services](https://minikube.sigs.k8s.io/docs/commands/service/) and [Pods](https://kubernetes.io/docs/concepts/workloads/pods/). And it will consume the first IP available in the [minikube node](https://minikube.sigs.k8s.io/docs/commands/node/#minikube-node).

With this information is very easy to preview which IPs will be available for the pods. In ours case from 10.244.0.3 to 10.244.0.6.

Note: assigning a Pod a static IP address is an anti-pattern in Kubernetes environments. This example is only for demonstrative purposes.
<hr>

References:<br>
[The Cargo Book](https://doc.rust-lang.org/cargo/)<br>
