# Tucano

An educational service scheduler and load balancer.

_(WIP - improve README.)_

## Running locally

Controller:

```
cargo run -p ctl
```

Worker:

```
cargo run -p worker -- --controller-addr '127.0.0.1:3000'
```
