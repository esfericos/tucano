use std::fmt;

use serde::{Deserialize, Serialize};

bty::brand!(
    pub type ServiceName = String;
);

#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkSpec {
    /// If `None`, won't expose any port.
    pub expose_port: Option<u16>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceSpec {
    pub name: ServiceName,
    pub network: NetworkSpec,
    pub scripts: Scripts,
    /// The maximum number of instances that Tucano is allowed to run for this
    /// service.
    pub concurrency: u32,
}

#[derive(Serialize, Deserialize)]
pub struct Scripts {
    /// The script that is used to build a new instance of this service.
    pub build_script: String,
    /// The script that is used to run an instance of this service.
    pub runtime_script: String,
    /// An optional string that is used to remove files associated with this
    /// service from a given worker node.
    pub teardown_script: Option<String>,
}

impl fmt::Debug for Scripts {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Scripts")
            .field("build", &"<...>")
            .field("runtime", &"<...>")
            .field("teardown", &self.teardown_script.as_ref().map(|_| "<...>"))
            .finish_non_exhaustive()
    }
}
