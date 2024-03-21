use serde::{Deserialize, Serialize};
use uuid::Uuid;

bty::brand!(
    pub type ConfigVersion = Uuid;
);

/// Controller configuration.
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    // TODO: Define configuration values.
}

/// Returns the current configuration.
pub struct GetConfigurationReq {}

/// Response for [`GetConfigurationReq`].
pub struct GetConfigurationRes {
    pub version: ConfigVersion,
    pub config: Config,
}

/// Defines a new configuration for the controller.
///
/// Implementors must implement optimistic locking.
#[derive(Debug, Serialize, Deserialize)]
pub struct PutConfigurationReq {
    /// The previous configuration version.
    pub version: ConfigVersion,
    /// The new configuration (will override the current one).
    pub config: Config,
}

/// Response for [`PutConfigurationReq`].
#[derive(Debug, Serialize, Deserialize)]
pub struct PutConfigurationRes {
    /// The new configuration version.
    pub version: ConfigVersion,
}
