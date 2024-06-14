use proto::common::instance::InstanceSpec;

use super::RunnerHandle;
#[allow(clippy::unused_async)]
pub async fn spawn_instance(spec: InstanceSpec, port: u16, _handle: RunnerHandle) {
    tokio::spawn(async move {
        match run_instance(spec, port).await {
            Ok(()) => todo!(),
            Err(_) => todo!(),
        }
    });
    todo!()
}
#[allow(clippy::unused_async)]
async fn run_instance(_spec: InstanceSpec, _port: u16) -> eyre::Result<()> {
    todo!();
}
