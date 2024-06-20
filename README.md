# Tucano

_(WIP - improve README.)_

An educational service scheduler and load balancer in a distributed system.

## Overview
Tucano is designed to manage and balance workloads across a cluster of nodes, leveraging Docker
containers for efficient resource usage. It consists of a central `Controller` and multiple
`Worker` nodes that work together to deploy, manage, and monitor services.

_(WIP - improve README.)_

## Running locally

### Prerequisites
Ensure you have and [Rust](https://www.rust-lang.org/) installed on your system.

### Steps

1. Start the controller:

```
cargo run -p ctl
```

2. Start a worker node:

```
cargo run -p worker -- --controller-addr '127.0.0.1'
```
