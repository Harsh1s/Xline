use serde::Deserialize;

use crate::config::engine_config::EngineConfig;

/// Storage Configuration
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
#[allow(clippy::module_name_repetitions)]
#[non_exhaustive]
pub struct StorageConfig {
    /// Engine Configuration
    #[serde(default = "EngineConfig::default")]
    pub engine: EngineConfig,
    /// Quota
    #[serde(default = "default_quota")]
    pub quota: u64,
}

impl StorageConfig {
    /// Create a new storage config
    #[inline]
    #[must_use]
    pub fn new(engine: EngineConfig, quota: u64) -> Self {
        Self { engine, quota }
    }
}

impl Default for StorageConfig {
    #[inline]
    fn default() -> Self {
        Self {
            engine: EngineConfig::default(),
            quota: default_quota(),
        }
    }
}

/// Default quota: 8GB
#[inline]
#[must_use]
pub fn default_quota() -> u64 {
    // 8 * 1024 * 1024 * 1024
    0x0002_0000_0000
}