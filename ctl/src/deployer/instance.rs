use std::net::IpAddr;

use proto::{
    common::instance::{self, InstanceId, InstanceSpec},
    ctl::deployer::RedeploymentPolicy,
    well_known::{MAX_INSTANCE_DEPLOY_RETRIES, MAX_INSTANCE_TERMINATION_RETRIES}, worker::runner::DeployInstanceRes,
};
use tracing::warn;

use crate::deployer::Deployer;

// Notice that we use less than OR EQUAL, so we start with 1.
const INITIAL_ATTEMPT: u8 = 1;

/// Computes the next state given the current state and a transition message.
///
/// Before returning the next state, this function may also schedule some
/// background worker task that will *eventually* produce another transition
/// message.
pub fn next(current: StateCtx, t: Transition, d: &mut Deployer) -> StateCtx {
    use instance::Status as s;
    use State::*;
    use Transition as t;
    match (current.state, t) {
        (Init, t::Deploy { worker_addr, spec }) => current.trans_into(Deploying {
            attempt: INITIAL_ATTEMPT,
            worker_addr,
            spec,
        }),

        (
            Deploying {
                attempt,
                worker_addr,
                spec,
            },
            t::Status(s::FailedToStart { error }),
        ) => {
            warn!(?error, "failed deploy attempt #{attempt}");
            if attempt <= MAX_INSTANCE_DEPLOY_RETRIES {
                let new_attempt = attempt + 1;
                current.trans_into(Deploying {
                    attempt: new_attempt,
                    worker_addr,
                    spec,
                })
            } else {
                current.trans_into(FailedToStart)
            }
        }

        (Deploying { .. }, t::Status(s::Started)) => {
            // TODO
            current.trans_into(Started)
        }

        (Deploying { .. }, t::Terminate(_)) => {
            // TODO
            current.trans_into(PreTerminating)
        }

        (PreTerminating, t::Status(s::Started)) => {
            // TODO
            current.trans_into(Terminating {
                attempt: INITIAL_ATTEMPT,
            })
        }

        (PreTerminating, t::Status(s::FailedToStart { .. })) => {
            // TODO
            current.trans_into(NeverStarted)
        }

        (Started, t::Status(s::Terminated { .. })) => {
            // TODO
            current.trans_into(UnexpectedTerminated)
        }

        (Started, t::Status(s::Crashed { .. })) => {
            // TODO
            current.trans_into(UnexpectedCrashed)
        }

        (Started, t::Status(s::Killed { .. })) => {
            // TODO
            current.trans_into(UnexpectedCrashed)
        }

        (Started, t::Terminate(_)) => {
            // TODO
            current.trans_into(Terminating {
                attempt: INITIAL_ATTEMPT,
            })
        }

        (Terminating { attempt }, t::FailedToTerminate) => {
            if attempt <= MAX_INSTANCE_TERMINATION_RETRIES {
                let new_attempt = attempt + 1;
                current.trans_into(Terminating {
                    attempt: new_attempt,
                })
            } else {
                current.trans_into(FailedToTerminate)
            }
        }

        (Terminating { .. }, t::Status(s::Terminated)) => {
            // TODO
            current.trans_into(Terminated)
        }

        (Terminating { .. }, t::Status(s::Crashed { .. })) => {
            // TODO
            current.trans_into(Crashed)
        }

        (Terminating { .. }, t::Status(s::Killed { .. })) => {
            // TODO
            current.trans_into(Crashed)
        }

        (s, t) => panic!("unexpected state transition `{t:?}` for current state `{s:?}`"),
    }
}

#[derive(Debug)]
pub struct StateCtx {
    state: State,
    /// The address of the worker in which this instance lives.
    worker_addr: IpAddr,
    id: InstanceId,
}

impl StateCtx {
    fn trans_into(mut self, next: State) -> StateCtx {
        self.state = next;
        self
    }

    fn trans_into_with_addr(mut self, next: State, next_addr: IpAddr) -> StateCtx {
        self.state = next;
        self.worker_addr = next_addr;
        self
    }
}

#[derive(Debug)]
pub enum State {
    Init,
    Deploying { attempt: u8, spec: InstanceSpec },
    FailedToStart,
    PreTerminating,
    NeverStarted,
    Started,
    UnexpectedTerminated,
    UnexpectedCrashed,
    Terminating { attempt: u8 },
    Terminated,
    Crashed,
    FailedToTerminate,
}

impl State {
    pub fn is_terminal(&self) -> bool {
        match self {
            State::Init => false,
            State::Deploying { .. } => false,
            State::FailedToStart => true,
            State::PreTerminating => false,
            State::NeverStarted => true,
            State::Started => false,
            State::UnexpectedTerminated => true,
            State::UnexpectedCrashed => true,
            State::Terminating { .. } => false,
            State::Terminated => true,
            State::Crashed => true,
            State::FailedToTerminate => true,
        }
    }
}

#[derive(Debug)]
pub enum Transition {
    Deploy {
        worker_addr: IpAddr,
        spec: InstanceSpec,
    },
    Terminate {
        worker_addr: IpAddr,
        id: InstanceId,
    },
    Status(instance::Status),
    // XX: For now, `FailedToTerminate` doesn't live in `instance::Status` since
    // we are not sure how to handle those corner error cases. In the future, we
    // may see fit to refactor the runner's container_rt implementation so that
    // it also reports termination failures. In such cases, however, the runner
    // must be able to recover from potential Docker failures (which are the
    // only sources of termination failures that the Tucano system can
    // encounter).
    FailedToTerminate,
}

fn deploy_instance(ctx: &StateCtx, spec: InstanceSpec, d: &mut Deployer) {
    let worker_addr = ctx.worker_addr;
    d.instance_task(ctx.id, move |h| async move {
        let result = h.worker_client
            .deploy_instance(worker_addr, spec)
            .await;
        match result {
            Ok(DeployInstanceRes {}) => Transition:: 
        }
        todo!()
    });
}
