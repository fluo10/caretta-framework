use std::path::PathBuf;

use crate::args::option::ConfigOptionArgs;
use crate::config::parsed::ParsedConfig;

use crate::mcp::client::Client;
use crate::types::AppInfo;
use crate::util::Mergeable;
use clap::Args;

/// Common option args for cli client.
#[derive(Args, Clone, Debug)]
pub struct ClientOptionArgs {
    #[command(flatten)]
    pub config: ConfigOptionArgs,
    /// If true, print log to stdout
    #[arg(short, long)]
    pub verbose: bool
}

impl ClientOptionArgs {
    /// Convert [`ConfigOptionArgs`] into [`ParsedConfig`]
    ///
    /// This function returns a merged [`ParsedConfig`] from the following two sources (The latter has priority).
    ///
    /// - Read from the configuration file.
    /// - Specified via arguments or environment variables
    pub async fn init_tracing_subscriber_and_spawn_client(self, app_info: AppInfo) -> Client {
        let app_name = app_info.name;
        let config = self
            .config
            .into_parsed_config(app_name)
            .with_default(app_name)
            .into_client_config(app_name)
            .unwrap();
        config.log.init_tracing_subscriber(self.verbose);
        config.spawn_client(app_info.info)
            .await
    }
}
