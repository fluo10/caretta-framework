use std::marker::PhantomData;

use crate::{types::AppInfo, util::RunnableCommand};
use clap::Args;

use crate::{
    args::option::ConfigOptionArgs,
    config::parsed::ParsedConfig,
    types::Verbosity,
};


/// Check config file is valid.
#[derive(Debug, Args)]
pub struct ConfigCheckCommandArgs {
    #[command(flatten)]
    config: ConfigOptionArgs,
}

impl RunnableCommand for ConfigCheckCommandArgs {
    #[tokio::main]
    async fn run(self, app_info: AppInfo) {
        let app_name = app_info.name;
        let config = self.config.into_parsed_config(app_name)
            .with_default(app_name)
            .with_database().await;
        #[cfg(feature = "desktop-client")]
        let _ = config.clone().into_client_config(app_name).unwrap();
        #[cfg(feature = "desktop-server")]
        let _ = config.clone().into_server_config(app_name).unwrap();
        println!("Ok")
    }
}
