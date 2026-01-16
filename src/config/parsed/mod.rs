//! Provides configs parsed from file, command-line args and environment valiables.
mod error;
mod log;
mod mcp;
mod p2p;
mod storage;
pub mod types;

#[cfg(feature = "client")]
use crate::config::ClientConfig;
#[cfg(feature = "desktop-server")]
use crate::config::ServerConfig;
#[cfg(feature = "server")]
use crate::config::{P2pConfig, StorageConfig};
use clap::Args;
pub use error::ParsedConfigError;
pub use log::ParsedLogConfig;
pub use mcp::ParsedMcpConfig;
pub use p2p::ParsedP2pConfig;

#[cfg(feature = "desktop-server")]
use rmcp::{RoleServer, Service};
use serde::{Deserialize, Serialize};
pub use storage::ParsedStorageConfig;

use crate::{
    config::{LogConfig, McpConfig},
    util::{Emptiable, Mergeable},
};
use std::{
    fmt::Display,
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};

#[derive(Args, Clone, Debug, Default, Deserialize, Serialize)]
pub struct ParsedConfig {
    #[command(flatten)]
    #[serde(default, skip_serializing_if = "ParsedStorageConfig::is_empty")]
    pub storage: ParsedStorageConfig,
    #[command(flatten)]
    #[serde(default, skip_serializing_if = "ParsedMcpConfig::is_empty")]
    pub mcp: ParsedMcpConfig,
    #[command(flatten)]
    #[serde(default, skip_serializing_if = "ParsedP2pConfig::is_empty")]
    pub p2p: ParsedP2pConfig,
    #[command(flatten)]
    #[serde(default, skip_serializing_if = "ParsedLogConfig::is_empty")]
    pub log: ParsedLogConfig,
}

/// A partial config parsed from config file, cli args, etc.
impl ParsedConfig {
    fn default(app_name: &'static str) -> Self {
        Self {
            storage: ParsedStorageConfig::default(app_name),
            mcp: ParsedMcpConfig::default(app_name),
            p2p: ParsedP2pConfig::empty(),
            log: ParsedLogConfig::default(),
        }
    }

    /// Fill empty configuration fields with default values and return.
    pub fn with_default(self, app_name: &'static str) -> Self {
        let mut result = Self::default(app_name);
        result.merge(self);
        result
    }

    /// Fill empty configuration fields with database values
    #[cfg(feature = "server")]
    pub async fn with_database(mut self) -> Self {
        use crate::entity::config;

        let db = self.to_storage_config().unwrap().open_database().await;
        let p2p_config = P2pConfig::from(
            config::Model::get_or_try_init(&Box::new(db))
                .await
                .unwrap(),
        );
        self.merge(ParsedP2pConfig::from(p2p_config));
        self
    }

    /// Remove server-only configurations
    #[cfg(feature = "client")]
    pub fn except_server_only_config(mut self) -> Self {
        self.p2p = ParsedP2pConfig::empty();
        self.storage = ParsedStorageConfig::empty();
        self
    }

    /// Build [`StorageConfig`] from own [`ParsedStorageConfig`]
    #[cfg(feature = "server")]
    pub fn to_storage_config(&self) -> Result<StorageConfig, ParsedConfigError> {
        self.storage.clone().try_into()
    }

    /// Build [`P2pConfig`] from own [`ParsedP2pConfig`]
    #[cfg(feature = "server")]
    pub fn to_p2p_config(&self) -> Result<P2pConfig, ParsedConfigError> {
        self.p2p.clone().try_into()
    }

    /// Build [`McpConfig`] from own [`ParsedMcpConfig`]
    pub fn to_mcp_config(&self) -> Result<McpConfig, ParsedConfigError> {
        self.mcp.clone().try_into()
    }
    /// Build [`LogConfig`] from own [`ParsedLogConfig`]
    pub fn to_log_config(&self) -> Result<LogConfig, ParsedConfigError> {
        self.log.clone().try_into()
    }
    /// Read or create target config file
    pub fn read_or_create_from_path<T>(path: T) -> Result<Self, ParsedConfigError>
    where
        T: AsRef<Path>,
    {
        if !path.as_ref().exists() {
            if let Some(x) = path.as_ref().parent() {
                std::fs::create_dir_all(x)?;
            };
            let _ = File::create(&path)?;
        }
        let mut file = File::open(path.as_ref())?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        let config: Self = toml::from_str(&content)?;
        Ok(config)
    }

    /// Get default config path
    pub fn default_config_path(app_name: &'static str) -> Result<PathBuf, ParsedConfigError> {
        const DEFAULT_FILE_NAME: &str = "config.toml";
        let mut path = dirs::config_local_dir().ok_or(ParsedConfigError::ConfigDir)?;
        path.push(app_name);
        path.push(DEFAULT_FILE_NAME);
        Ok(path)
    }

    /// Read or create target config file at the default config path
    pub fn read_or_create(app_name: &'static str) -> Result<Self, ParsedConfigError> {
        let config_dir = Self::default_config_path(app_name)?;
        Self::read_or_create_from_path(config_dir)
    }

    /// Create [`ServerConfig`] from `ParsedConfig`
    #[cfg(feature = "desktop-server")]
    pub fn into_server_config(
        self,
        app_name: &'static str,
    ) -> Result<ServerConfig, ParsedConfigError> {
        Ok(ServerConfig {
            log: self.to_log_config()?,
            mcp: self.to_mcp_config()?,
            p2p: self.to_p2p_config()?,
            storage: self.to_storage_config()?,
        })
    }

    #[cfg(feature = "client")]
    pub fn into_client_config(
        self,
        app_name: &'static str,
    ) -> Result<ClientConfig, ParsedConfigError> {
        let config = self.as_ref();
        let mcp = config.to_mcp_config()?;
        let log = config.to_log_config()?;
        Ok(ClientConfig { mcp, log })
    }
}

impl AsRef<ParsedConfig> for ParsedConfig {
    fn as_ref(&self) -> &ParsedConfig {
        self
    }
}

impl Emptiable for ParsedConfig {
    fn empty() -> Self {
        Self {
            p2p: ParsedP2pConfig::empty(),
            storage: ParsedStorageConfig::empty(),
            mcp: ParsedMcpConfig::empty(),
            log: ParsedLogConfig::empty(),
        }
    }

    fn is_empty(&self) -> bool {
        self.p2p.is_empty() && self.mcp.is_empty() && self.storage.is_empty() && self.log.is_empty()
    }
}

impl Mergeable for ParsedConfig {
    fn merge(&mut self, other: Self) {
        self.p2p.merge(other.p2p);
        self.mcp.merge(other.mcp);
        self.storage.merge(other.storage);
        self.log.merge(other.log);
    }
}

impl Display for ParsedConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", toml::to_string(self).map_err(|_| std::fmt::Error)?)
    }
}
impl Mergeable<ParsedP2pConfig> for ParsedConfig {
    fn merge(&mut self, other: ParsedP2pConfig) {
        self.p2p.merge(other);
    }
}
