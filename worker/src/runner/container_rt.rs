use std::{collections::HashMap, sync::Arc};

use bollard::{
    container::{
        Config, CreateContainerOptions, KillContainerOptions, StartContainerOptions,
        WaitContainerOptions,
    },
    errors::Error as BollardError,
    secret::{ContainerCreateResponse, ContainerWaitExitError, ContainerWaitResponse, HostConfig},
    Docker,
};
use futures_util::stream::StreamExt;
use proto::{
    common::instance::{InstanceId, InstanceSpec, Status},
    well_known::GRACEFUL_SHUTDOWN_DEADLINE,
};
use tracing::{error, instrument, trace};

use super::RunnerHandle;

#[derive(Clone)]
pub struct ContainerRuntime {
    docker: Arc<Docker>,
}

impl ContainerRuntime {
    pub fn new(docker: Arc<Docker>) -> Self {
        ContainerRuntime { docker }
    }

    #[instrument(skip_all, fields(instance_id = ?spec.instance_id))]
    pub async fn run_instance_lifecycle(
        &self,
        spec: InstanceSpec,
        port: u16,
        handle: RunnerHandle,
    ) {
        let container_name = Self::create_container_name(spec.instance_id);
        trace!(?spec, container_name, "running instance lifecycle");

        if let Err(error) = self
            .create_and_run(&spec, port, container_name.clone())
            .await
        {
            error!(?error, "failed to create/run container");
            let error = error.to_string();
            handle
                .report_instance_status(spec.instance_id, Status::FailedToStart { error })
                .await;
            return;
        }

        // TODO: Add health check to verify whether the service is running
        trace!("container running");
        handle
            .report_instance_status(spec.instance_id, Status::Started)
            .await;

        match self
            .wait_container(spec.instance_id)
            .await
            .expect("infallible operation")
        {
            ExitStatus::Terminated => {
                trace!("container terminated");
                handle
                    .report_instance_status(spec.instance_id, Status::Terminated)
                    .await;
            }
            ExitStatus::Crashed { status, error } => {
                error!(status, instance_id = %spec.instance_id, "container crashed");
                handle
                    .report_instance_status(spec.instance_id, Status::Crashed { error })
                    .await;
            }
        }
    }

    pub async fn terminate_instance(&self, id: InstanceId) {
        if let Err(e) = self.kill_container(id, "SIGTERM").await {
            error!(%e, "error when killing instance (term)");
        }

        let timeout_res = tokio::time::timeout(GRACEFUL_SHUTDOWN_DEADLINE, self.wait_container(id));

        match timeout_res.await {
            // Container has been gracefully terminated.
            Ok(_) => (),
            // Container failed to terminate within given deadline.
            Err(_) => {
                if let Err(e) = self.kill_container(id, "SIGKILL").await {
                    error!(%e, "error when killing instance (kill)");
                }
            }
        }
    }

    async fn create_and_run(
        &self,
        spec: &InstanceSpec,
        port: u16,
        name: String,
    ) -> eyre::Result<()> {
        let create_response = self.create_container(spec, port, name.clone()).await?;
        trace!("successfully `create` operation");

        self.run_container(create_response).await?;
        trace!("successfully `run` operation");

        Ok(())
    }

    async fn run_container(&self, create_response: ContainerCreateResponse) -> eyre::Result<()> {
        self.docker
            .start_container(&create_response.id, None::<StartContainerOptions<String>>)
            .await?;

        Ok(())
    }

    async fn create_container(
        &self,
        spec: &InstanceSpec,
        port: u16,
        name: String,
    ) -> eyre::Result<ContainerCreateResponse> {
        let config = Self::create_container_config(spec.clone(), port);

        let options = Some(CreateContainerOptions {
            name,
            platform: None,
        });
        let create_response = self.docker.create_container(options, config).await?;

        Ok(create_response)
    }

    async fn wait_container(&self, id: InstanceId) -> eyre::Result<ExitStatus> {
        let ct_name = Self::create_container_name(id);
        let options = Some(WaitContainerOptions {
            condition: "not-running",
        });
        let mut response_stream = self.docker.wait_container(&ct_name, options);
        let Some(result) = response_stream.next().await else {
            eyre::bail!("wait_container didn't respond");
        };

        match result {
            Ok(res) if res.status_code == 0 => Ok(ExitStatus::Terminated),
            // Although this `Ok` variant is impossible as per the library's
            // source code, the type signature still allows it, so we handle
            // it here. The library maps the `Ok` with non-0 exit status code
            // to the OR-ed `Err` case.
            Ok(ContainerWaitResponse {
                status_code: status,
                error: Some(ContainerWaitExitError { message: Some(m) }),
            })
            | Err(BollardError::DockerContainerWaitError {
                error: m,
                code: status,
            }) => Ok(ExitStatus::Crashed { status, error: m }),
            Ok(ContainerWaitResponse {
                status_code,
                error: _,
            }) => Ok(ExitStatus::Crashed {
                status: status_code,
                error: "unknown".into(),
            }),
            Err(e) => Err(e.into()),
        }
    }

    async fn kill_container(&self, id: InstanceId, signal: &str) -> eyre::Result<()> {
        let ct_name = Self::create_container_name(id);
        self.docker
            .kill_container(&ct_name, Some(KillContainerOptions { signal }))
            .await?;
        Ok(())
    }

    fn create_container_config(spec: InstanceSpec, port: u16) -> Config<String> {
        const HOST: &str = "0.0.0.0";

        Config {
            image: Some(spec.image.0),
            exposed_ports: Some(HashMap::from([(
                format!("{port}/tcp"),
                #[allow(clippy::zero_sized_map_values)]
                HashMap::default(),
            )])),
            env: Some(vec![format!("PORT={port}"), format!("HOST={HOST}")]),
            host_config: Some(HostConfig {
                auto_remove: Some(true),
                // FIXME: These aren't working right now.
                //
                // cpu_shares: Some(spec.resource_config.cpu_shares),
                // memory: Some(spec.resource_config.memory_limit),
                port_bindings: Some({
                    let mut map = HashMap::new();
                    map.insert(
                        format!("{port}/tcp"),
                        Some(vec![bollard::models::PortBinding {
                            host_ip: Some(HOST.to_string()),
                            host_port: Some(port.to_string()),
                        }]),
                    );
                    map
                }),
                ..Default::default()
            }),
            ..Default::default()
        }
    }

    fn create_container_name(id: InstanceId) -> String {
        format!("instance-{id}")
    }
}

enum ExitStatus {
    Terminated,
    Crashed { status: i64, error: String },
}
