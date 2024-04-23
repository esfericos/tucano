# Tucano

## Components

A Tucano cluster is comprised of a single "controller" and multiple "worker"
nodes. A controller node is responsible for managing the cluster and providing
load-balancing facilities. Worker nodes are responsible for running service
workloads.

Their main components are:

```mermaid
graph TB
    subgraph Controller
        direction TB

        http --> deployer
        http --> worker_mgr

        worker_mgr --> notifier
        worker_mgr <--> discovery

        deployer <--> discovery

        discovery --> balancer
    end

    subgraph Worker
        direction LR

        subgraph Monitor
            collector --> scheduler
        end

        subgraph Runner
            builder
            supervisor
        end

        subgraph Services
            subgraph Service
                health
            end
        end
    end
```

### Brief definitions

- **Controller**
  - `http` -> receives external requests from worker nodes or the system
    administrator's CLI and routes them to the appropriate component.
  - `deployer` -> handles service deployments and service teardown ("undeploy")
    requests.
  - `worker_mgr`
    - Receives and processes CPU and memory metrics from worker nodes.
    - Handles "dead" worker nodes.
    - Handles any errors that may occur within a worker node. For example, a
      failed deploy attempt.
  - `discovery` -> acts as a central database that records all available worker
    nodes and services that are currently running on the system.
  - `balancer` -> handles external-user requests and routes them to the
    appropriate service which is running on a worker node.
- **Worker**
  - **Monitor**
    - `collector` -> collects system metrics (average CPU and memory) from the
      corresponding worker node.
    - `scheduler` -> periodically calls the collector module to fetch the latest
      metrics, and sends them to the controller.
  - **Runner**
    - `builder` -> Builds a new service on the corresponding worker node
      (essentially, it is a glorified build script runner).
    - `supervisor` -> Runs a service's "runtime script" to properly start the
      service, and supervises such a service to handle errors, retries, etc. It
      may also optionally call a health check endpoint for the corresponding
      service.

# Deployment seq. diagram

```mermaid
sequenceDiagram
    %% Alice->>+John: Hello John, how are you?
    %% Alice->>+John: John, can you hear me?
    %% John-->>-Alice: Hi Alice, I can hear you!
    %% John-->>-Alice: I feel great!
    actor SysAdmin
    participant deployer as ctl::deployer
    participant discovery as ctl::discovery
    participant wrk_mgr as ctl::worker_mgr

    participant runner as wrk::runner
    participant builder as wrk::builder
    participant supervisor as wrk::supervisor

    SysAdmin ->>+ deployer: New deploy request (via CLI)
    deployer ->>+ discovery: Create new deployment
    discovery ->>- deployer: Deployment ID
    deployer ->>- SysAdmin: Deployment ID

    deployer ->>+ discovery: Fetch available workers
    discovery ->>- deployer: 

    deployer ->> deployer: Select worker

    deployer -->>+ runner: Start deployment

    runner ->>+ builder: Execute build script
    builder ->>- runner: Report status

    alt build failed
        runner -->> wrk_mgr: Report build failure
        wrk_mgr -->> discovery: Record build failed status
    else build ok
        runner ->>+ supervisor: Start service
        supervisor ->>- runner: Report status

        alt service started
            runner -->> wrk_mgr: Report service running
            wrk_mgr -->> discovery: Record running status

            opt service crashed
                runner -->> wrk_mgr: Report failed
                wrk_mgr -->> discovery: Record failed status
            end
        else service failed to start
            runner -->> wrk_mgr: Report failure
            wrk_mgr -->> discovery: Record failed status
        end
    end
```

# TODO

- Add sequence diagrams for communication patterns.
- Deployment options, etc.
