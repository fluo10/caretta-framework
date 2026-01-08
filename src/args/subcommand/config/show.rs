use std::marker::PhantomData;

use crate::{config::parsed::ParsedConfig, types::AppInfo, util::RunnableCommand};
use clap::Args;

use crate::args::option::ConfigOptionArgs;

/// Print active config list.
#[derive(Debug, Args)]
pub struct ConfigShowCommandArgs {
    #[command(flatten)]
    config: ConfigOptionArgs,
}

impl RunnableCommand for ConfigShowCommandArgs {
    #[tokio::main]
    async fn run(self, app_info : AppInfo) {
        let app_name = app_info.name;
        let mut config = self.config.into_parsed_config(app_name);
        config = config.with_default(app_name);
        #[cfg(feature = "desktop-server")]
        {
            config = config.with_database().await;
        }
        println!("{}", config)
    }
}
