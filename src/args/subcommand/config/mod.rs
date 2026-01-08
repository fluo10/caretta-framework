mod check;
use check::*;

mod show;
use clap::{Args, Subcommand};
use show::*;

use crate::util::RunnableCommand;

#[derive(Args, Debug)]
pub struct ConfigCommandArgs {
    #[command(subcommand)]
    command: ConfigSubcommand,
}

impl RunnableCommand for ConfigCommandArgs {
    fn run(self, app_info: crate::types::AppInfo) {
        self.command.run(app_info)
    }
}
#[derive(Subcommand, Debug)]
pub enum ConfigSubcommand {
    Check(ConfigCheckCommandArgs),
    Show(ConfigShowCommandArgs)
}

impl RunnableCommand for ConfigSubcommand {
    fn run(self, app_info: crate::types::AppInfo) {
        match self {
            ConfigSubcommand::Check(x) => x.run(app_info),
            ConfigSubcommand::Show(x) => x.run(app_info),
        }
    }
}