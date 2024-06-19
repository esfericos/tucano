//! Worker allocation algorithms.

use std::net::IpAddr;

use proto::common::instance::InstanceId;
use rand::seq::SliceRandom;
use uuid::Uuid;

use crate::worker_mgr::WorkerDetails;

/// Randomly allocates, using an uniform distribution, instances for the give
/// amount of instances and the provided pool of `workers`.
pub fn rand_many(
    workers: &[WorkerDetails],
    instances: u32,
) -> impl Iterator<Item = (InstanceId, IpAddr)> + '_ {
    workers
        .choose_multiple(&mut rand::thread_rng(), instances as usize)
        .map(|w| (InstanceId(Uuid::now_v7()), w.addr))
}

/// Randomly allocates a single instance from the provided pool of `workers`.
pub fn rand_single(workers: &[WorkerDetails]) -> (InstanceId, IpAddr) {
    rand_many(workers, 1).next().unwrap()
}
