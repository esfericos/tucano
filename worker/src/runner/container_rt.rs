use std::{collections::HashMap, sync::Arc};

use bollard::{
    container::{Config, CreateContainerOptions, StartContainerOptions, WaitContainerOptions},
    secret::{ContainerCreateResponse, ContainerWaitExitError, ContainerWaitResponse, HostConfig},
    Docker,
};
use futures_util::stream::StreamExt;
use proto::common::instance::{InstanceSpec, Status};

use super::RunnerHandle;

#[derive(Clone)]
pub struct ContainerRuntime {
    docker: Arc<Docker>,
}

impl ContainerRuntime {
    pub fn new(docker: Arc<Docker>) -> Self {
        ContainerRuntime { docker }
    }

    pub fn spawn_instance(&self, spec: InstanceSpec, port: u16, handle: RunnerHandle) {
        let this = self.clone();
        tokio::spawn(async move {
            let container_name = Self::create_container_name(&spec);

            if let Err(e) = this
                .create_and_run(&spec, port, container_name.clone())
                .await
            {
                let error = e.to_string();
                handle
                    .report_instance_status(Status::FailedToStart { error })
                    .await;
                return;
            }

            // healthcheck verifies if service is running on established `PORT`
            handle.report_instance_status(Status::Started).await;

            if let Err(e) = this.wait_container(&container_name).await {
                let error = e.to_string();
                handle
                    .report_instance_status(Status::Crashed { error })
                    .await;
                return;
            }

            handle.report_instance_status(Status::Terminated).await;
        });
    }

    async fn create_and_run(
        &self,
        spec: &InstanceSpec,
        port: u16,
        name: String,
    ) -> eyre::Result<()> {
        let create_response = self.create_container(spec, port, name.clone()).await?;

        self.run_container(create_response).await?;
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
        let config = self.create_container_config(spec.clone(), port);

        let options = Some(CreateContainerOptions {
            name,
            platform: Some("linux/x86_64".to_string()),
        });
        let create_response = self.docker.create_container(options, config).await?;

        Ok(create_response)
    }

    async fn wait_container(&self, name: &str) -> eyre::Result<()> {
        let options = Some(WaitContainerOptions {
            condition: "not-running",
        });

        let mut response_stream = self.docker.wait_container(name, options);
        let Some(result) = response_stream.next().await else {
            eyre::bail!("wait_container didn't respond");
        };

        match result {
            Ok(res) if res.status_code == 0 => Ok(()),
            Ok(ContainerWaitResponse {
                status_code,
                error: Some(ContainerWaitExitError { message: Some(m) }),
            }) => Err(eyre::eyre!("Container exited due to: {m} - {status_code}")),
            Ok(ContainerWaitResponse {
                status_code,
                error: _,
            }) => Err(eyre::eyre!(
                "Container exited due to unknown error - {status_code}"
            )),
            Err(e) => Err(e.into()),
        }
    }

    #[allow(clippy::unused_self)]
    #[allow(clippy::zero_sized_map_values)]
    fn create_container_config(&self, spec: InstanceSpec, port: u16) -> Config<String> {
        Config {
            image: Some(spec.image.0),
            exposed_ports: Some({
                let mut map = HashMap::new();
                map.insert(format!("{port}/tcp"), HashMap::default());
                map
            }),
            env: Some(vec![format!("PORT={port}")]),
            host_config: Some(HostConfig {
                cpu_shares: Some(spec.resource_config.cpu_shares),
                memory: Some(spec.resource_config.memory_limit),
                port_bindings: Some({
                    let mut map = HashMap::new();
                    map.insert(
                        format!("{port}/tcp"),
                        Some(vec![bollard::models::PortBinding {
                            host_ip: Some("0.0.0.0".to_string()),
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

    fn create_container_name(spec: &InstanceSpec) -> String {
        format!("instance-{}", spec.instance_id.0)
    }
}
