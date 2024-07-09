# Pre-steps

Note that if you already tested and changed some source code, you'll have to
build without the cache. E.g.

```bash
docker compose -f ./tests/perf-analysis/<...>/docker-compose.yml build --no-cache
```

Build the image

```bash
docker image build -t lffg/number-fact:latest ./tests/containers/number-fact
```

# Setup network

The compose file uses an externally-managed Docker network named
`tucano-cluster-net`. Make sure it is created before starting the environments.

```bash
docker network create tucano-cluster-net
```

# Single node

Start:

```bash
docker compose -f ./tests/perf-analysis/single-node/docker-compose.yml up
```

In another terminal session:

```bash
k6 run ./tests/perf-analysis/test.js
```

# Tucano multi node

```bash
docker compose -f ./tests/perf-analysis/tuc-multi-node/docker-compose.yml up
```

Ensure that ctl log shows that 3 worker nodes joined the cluster.

Deploy the application:

```bash
./tucano -c localhost service deploy --id="number-fact" --image="lffg/number-fact" --public --concurrency="3"
```

Ensure that all deployments went right.

In another terminal session:

```bash
k6 run ./tests/perf-analysis/test.js
```

# Tucano multi node (4x)

Same steps as multi node (previous test), but different docker-compose file:

```bash
docker compose -f ./tests/perf-analysis/tuc-multi-node-4x/docker-compose.yml up
```
