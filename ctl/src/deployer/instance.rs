use std::net::IpAddr;

use proto::{
    common::instance::{self, InstanceId, InstanceSpec},
    ctl::deployer::DeploymentId,
    well_known::{MAX_INSTANCE_DEPLOY_RETRIES, MAX_INSTANCE_TERMINATION_RETRIES},
    worker::runner::{DeployInstanceRes, TerminateInstanceRes},
};
use tracing::{instrument, warn};
use utils::fmt::ElideDebug;

use crate::deployer::Deployer;

// Notice that we use less than OR EQUAL, so we start with 1.
const INITIAL_ATTEMPT: u8 = 1;

/// Computes the next state given the current state and a transition message.
///
/// Before returning the next state, this function may also schedule some
/// background worker task that will *eventually* produce another transition
/// message.
#[
    // Notice that through this span we log eventual errors.
    instrument(skip(d))
]
pub fn next(d: &mut Deployer, current: StateCtx, t: Transition) -> StateCtx {
    use instance::Status as s;
    use State::*;
    use Transition as t;

    match (current.state.clone(), t) {
        (Init, t::Deploy { spec }) => {
            schedule_instance_deployment(d, &current, spec.get().clone());
            current.trans_into(Deploying {
                attempt: INITIAL_ATTEMPT,
                spec,
            })
        }

        (Deploying { attempt, spec }, t::FailedToDeploy(_error)) => {
            warn!("failed to deploy (deployment attempt #{attempt}");
            schedule_instance_deployment_reattempt(d, current, attempt, spec.get().clone())
        }

        (Deploying { attempt, spec }, t::Status(s::FailedToStart { error: _ })) => {
            warn!("failed to start (deployment attempt #{attempt})");
            schedule_instance_deployment_reattempt(d, current, attempt, spec.get().clone())
        }

        (Deploying { .. }, t::Status(s::Started)) => {
            //
            current.trans_into(Started)
        }

        (Deploying { .. }, t::Terminate) => {
            //
            current.trans_into(PreTerminating)
        }

        (PreTerminating, t::Status(s::Started)) => {
            // TODO
            current.trans_into(Terminating {
                attempt: INITIAL_ATTEMPT,
            })
        }

        (PreTerminating, t::FailedToDeploy(_error)) => {
            warn!("failed to deploy instance");
            current.trans_into(NeverStarted)
        }

        (PreTerminating, t::Status(s::FailedToStart { .. })) => {
            warn!("failed to start instance");
            // TODO
            current.trans_into(NeverStarted)
        }

        (Started, t::Status(s::Terminated { .. })) => {
            warn!("instance unexpectedly terminated");
            current.trans_into(UnexpectedTerminated)
        }

        (Started, t::Status(s::Crashed { .. })) => {
            warn!("instance unexpectedly crashed");
            current.trans_into(UnexpectedCrashed)
        }

        (Started, t::Status(s::Killed { .. })) => {
            warn!("instance was killed");
            current.trans_into(UnexpectedCrashed)
        }

        (Started, t::Terminate) => {
            schedule_instance_termination(d, &current);
            current.trans_into(Terminating {
                attempt: INITIAL_ATTEMPT,
            })
        }

        (Terminating { attempt }, t::FailedToTerminate(_error)) => {
            warn!("failed to terminate (termination attempt #{attempt})");
            schedule_instance_termination_reattempt(d, current.clone(), attempt)
        }

        (Terminating { .. }, t::Status(s::Terminated)) => {
            //
            current.trans_into(Terminated)
        }

        (Terminating { .. }, t::Status(s::Crashed { .. })) => {
            warn!("instance crashed");
            current.trans_into(Crashed)
        }

        (Terminating { .. }, t::Status(s::Killed { .. })) => {
            warn!("instance was killed");
            current.trans_into(Crashed)
        }

        (s, t) => panic!("unexpected state transition `{t:?}` for current state `{s:?}`"),
    }
}

#[derive(Debug, Clone)]
pub struct StateCtx {
    state: State,
    id: InstanceId,
    /// The address of the worker in which this instance lives.
    worker_addr: IpAddr,
    deployment_id: DeploymentId,
}

impl StateCtx {
    pub fn new_init(id: InstanceId, worker_addr: IpAddr, deployment_id: DeploymentId) -> Self {
        StateCtx {
            state: State::Init,
            id,
            worker_addr,
            deployment_id,
        }
    }

    pub fn state(&self) -> &State {
        &self.state
    }

    #[allow(dead_code)]
    pub fn deployment_id(&self) -> DeploymentId {
        self.deployment_id
    }

    fn trans_into(mut self, next: State) -> StateCtx {
        self.state = next;
        self
    }
}

#[derive(Debug, Clone)]
pub enum State {
    Init,
    Deploying {
        attempt: u8,
        spec: ElideDebug<InstanceSpec>,
    },
    FailedToStart,
    PreTerminating,
    NeverStarted,
    Started,
    UnexpectedTerminated,
    UnexpectedCrashed,
    Terminating {
        attempt: u8,
    },
    Terminated,
    Crashed,
    FailedToTerminate,
}

impl State {
    #[allow(clippy::match_same_arms)]
    pub fn kind(&self) -> TerminalKind {
        use TerminalKind::*;
        match self {
            State::Init => NonTerminal,
            State::Deploying { .. } => NonTerminal,
            State::FailedToStart => UnsuccessfulTerminal,
            State::PreTerminating => NonTerminal,
            State::NeverStarted => UnsuccessfulTerminal,
            State::Started => NonTerminal,
            State::UnexpectedTerminated => UnsuccessfulTerminal,
            State::UnexpectedCrashed => UnsuccessfulTerminal,
            State::Terminating { .. } => NonTerminal,
            State::Terminated => SuccessfulTerminal,
            State::Crashed => UnsuccessfulTerminal,
            State::FailedToTerminate => UnsuccessfulTerminal,
        }
    }
}

/// Describes whether a state machine state is terminal or not, and if a
/// terminal state is (or not) successful.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[allow(clippy::enum_variant_names)]
pub enum TerminalKind {
    NonTerminal,
    SuccessfulTerminal,
    UnsuccessfulTerminal,
}

#[derive(Debug)]
pub enum Transition {
    Deploy {
        spec: ElideDebug<InstanceSpec>,
    },
    #[allow(dead_code)]
    Terminate,
    Status(instance::Status),
    // XX: For now, `FailedToTerminate` doesn't live in `instance::Status` since
    // we are not sure how to handle those corner error cases. In the future, we
    // may see fit to refactor the runner's container_rt implementation so that
    // it also reports termination failures. In such cases, however, the runner
    // must be able to recover from potential Docker failures (which are the
    // only sources of termination failures that the Tucano system can
    // encounter).
    FailedToTerminate(eyre::Report),
    FailedToDeploy(eyre::Report),
}

fn schedule_instance_deployment(d: &mut Deployer, ctx: &StateCtx, spec: InstanceSpec) {
    let worker_addr = ctx.worker_addr;
    d.instance_task(ctx.id, move |h| async move {
        let result = h.worker_client.deploy_instance(worker_addr, spec).await;
        match result {
            Ok(DeployInstanceRes {}) => None,
            Err(error) => Some(Transition::FailedToDeploy(error)),
        }
    });
}

fn schedule_instance_deployment_reattempt(
    d: &mut Deployer,
    current: StateCtx,
    attempt: u8,
    spec: InstanceSpec,
) -> StateCtx {
    if attempt <= MAX_INSTANCE_DEPLOY_RETRIES {
        schedule_instance_deployment(d, &current, spec.clone());

        let new_attempt = attempt + 1;
        current.trans_into(State::Deploying {
            attempt: new_attempt,
            spec: spec.into(),
        })
    } else {
        current.trans_into(State::FailedToStart)
    }
}

fn schedule_instance_termination(d: &mut Deployer, ctx: &StateCtx) {
    let worker_addr = ctx.worker_addr;
    let id = ctx.id;
    d.instance_task(ctx.id, move |h| async move {
        let result = h.worker_client.terminate_instance(worker_addr, id).await;
        match result {
            Ok(TerminateInstanceRes {}) => None,
            Err(error) => Some(Transition::FailedToTerminate(error)),
        }
    });
}

fn schedule_instance_termination_reattempt(
    d: &mut Deployer,
    current: StateCtx,
    attempt: u8,
) -> StateCtx {
    if attempt <= MAX_INSTANCE_TERMINATION_RETRIES {
        schedule_instance_termination(d, &current);

        let new_attempt = attempt + 1;
        current.trans_into(State::Terminating {
            attempt: new_attempt,
        })
    } else {
        current.trans_into(State::FailedToTerminate)
    }
}
