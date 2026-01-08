use clap::Args;
use serde::{Deserialize, Serialize};

use crate::types::{DevicePublicKey, DeviceSecretKey};
use crate::util::{Emptiable, Mergeable};

#[derive(Args, Clone, Debug, Default, Deserialize, Serialize)]
pub struct ParsedP2pConfig {
    #[serde(skip_serializing)]
    #[arg(long = "p2p-secret-key", env = "P2P_SECRET_KEY")]
    pub secret_key: Option<DeviceSecretKey>,
    #[serde(skip_deserializing)]
    #[arg(skip)]
    pub public_key: Option<DevicePublicKey>,
    #[arg(long = "p2p-enable-n0", env = "P2P_ENABLE_N0")]
    pub enable_n0: Option<bool>,
    #[arg(long = "p2p-enable-mdns", env = "P2P_ENABLE_MDNS")]
    pub enable_mdns: Option<bool>,
}

impl Emptiable for ParsedP2pConfig {
    fn empty() -> Self {
        Self {
            secret_key: None,
            public_key: None,
            enable_mdns: None,
            enable_n0: None,
        }
    }

    fn is_empty(&self) -> bool {
        self.secret_key.is_none()
            && self.enable_mdns.is_none()
            && self.enable_n0.is_none()
            && self.public_key.is_none()
    }
}

impl Mergeable for ParsedP2pConfig {
    fn merge(&mut self, mut other: Self) {
        if let Some(x) = other.secret_key.take() {
            let _ = self.secret_key.insert(x);
        };
        if let Some(x) = other.enable_n0.take() {
            let _ = self.enable_n0.insert(x);
        };
        if let Some(x) = other.enable_mdns.take() {
            let _ = self.enable_mdns.insert(x);
        };
        if let Some(x) = other.public_key.take() {
            let _ = self.public_key.insert(x);
        }
    }
}

#[cfg(feature = "server")]
mod server {
    use std::array::TryFromSliceError;

    use super::*;
    use crate::{config::P2pConfig, config::parsed::ParsedConfigError};
    use iroh::SecretKey;
    impl TryFrom<ParsedP2pConfig> for P2pConfig {
        type Error = ParsedConfigError;
        fn try_from(raw: ParsedP2pConfig) -> Result<P2pConfig, Self::Error> {
            Ok(P2pConfig {
                secret_key: raw
                    .secret_key
                    .ok_or(ParsedConfigError::MissingConfig("p2p.secret_key"))?,
                enable_n0: raw
                    .enable_n0
                    .ok_or(ParsedConfigError::MissingConfig("p2p.enable_n0"))?,
                enable_mdns: raw
                    .enable_mdns
                    .ok_or(ParsedConfigError::MissingConfig("p2p.enable_mdns"))?,
            })
        }
    }
    impl From<P2pConfig> for ParsedP2pConfig {
        fn from(config: P2pConfig) -> Self {
            Self {
                secret_key: Some(config.secret_key.clone()),
                public_key: Some(config.secret_key.public_key()),
                enable_mdns: Some(config.enable_mdns),
                enable_n0: Some(config.enable_n0),
            }
        }
    }
}
