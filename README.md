# Tucano

An educational service scheduler and load balancer.

_(WIP - improve README.)_

## Manually run locally with Docker

Build container and worker images:

```bash
docker build -f dockerfiles/node.dockerfile --build-arg='CRATE=ctl' --tag ctl .
docker build -f dockerfiles/node.dockerfile --build-arg='CRATE=worker' --tag worker .
```

The images don't define a Docker `ENTRYPOINT` rules. The container caller must
manually execute the binary that is in `/usr/local/bin`. It may be `ctl` or
`worker`.

Create a network that will be shared by the controller and worker nodes:

```bash
docker network create tucano-net
```

Run the controller:

```bash
docker container run --rm --network tucano-net --name ctl --entrypoint '/usr/local/bin/ctl' ctl
```

Fetch its designated IP address:

```bash
export TUC_CTL_IP="$(docker container inspect ctl --format '{{range.NetworkSettings.Networks}}{{.IPAddress}}{{end}}')"
echo "$TUC_CTL_IP"
```

Add as many worker nodes as you want:

```bash
docker container run --rm --network tucano-net --entrypoint '/usr/local/bin/worker' worker "--controller-addr=$TUC_CTL_IP"
```
