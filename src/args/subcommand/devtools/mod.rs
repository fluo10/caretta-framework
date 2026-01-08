mod ping;

use crate::{types::AppInfo, util::RunnableCommand};

pub use ping::DevPingCommandArgs;

use clap::{Args, Subcommand};

#[derive(Debug, Args)]
pub struct DevCommandArgs {
    #[command(subcommand)]
    command: DevSubcommand,
}

impl RunnableCommand for DevCommandArgs {
    fn run(self, app_info: AppInfo) {
        self.command.run(app_info)
    }
}

#[derive(Debug, Subcommand)]
enum DevSubcommand {
    Ping(DevPingCommandArgs),
}

impl RunnableCommand for DevSubcommand {
    fn run(self, app_info: AppInfo) {
        match self {
            Self::Ping(x) => x.run(app_info),
        }
    }
}
