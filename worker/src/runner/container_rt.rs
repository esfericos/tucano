use std::collections::HashMap;

use bollard::{
    container::{Config, CreateContainerOptions, StartContainerOptions},
    secret::HostConfig,
    Docker,
};
use proto::common::instance::{InstanceId, InstanceSpec};

use super::RunnerHandle;

#[derive(Clone)]
pub struct ContainerRuntime {
    docker: Docker,
    containers: HashMap<InstanceId, String>,
}

impl ContainerRuntime {
    pub fn new() -> Self {
        let d = Docker::connect_with_defaults().unwrap();
        ContainerRuntime {
            docker: d,
            containers: HashMap::default(),
        }
    }

    #[allow(clippy::unused_async)]
    pub async fn spawn_instance(&mut self, spec: InstanceSpec, port: u16, _handle: RunnerHandle) {
        let mut rt_clone = self.clone();
        tokio::spawn(async move {
            match rt_clone.run_instance(spec, port).await {
                Ok(()) => todo!(),
                Err(_) => todo!(),
            }
        });
        todo!()
    }

    #[allow(clippy::unused_async)]
    async fn run_instance(&mut self, spec: InstanceSpec, port: u16) -> eyre::Result<()> {
        let config = self.create_container_config(spec.clone(), port);
        let container_name = format!("instance-{}", spec.instance_id.0);

        let options = Some(CreateContainerOptions {
            name: container_name,
            platform: Some("linux/x86_64".to_string()),
        });
        let create_response = self.docker.create_container(options, config).await?;

        self.docker
            .start_container(&create_response.id, None::<StartContainerOptions<String>>)
            .await?;

        self.containers.insert(spec.instance_id, create_response.id);

        Ok(())
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
}
