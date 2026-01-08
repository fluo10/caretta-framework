use caretta_framework::{args::subcommand::DevCommandArgs, types::AppInfo, util::RunnableCommand};
use caretta_framework_example_core::APP_NAME;
use clap::{Parser, Subcommand};
use rmcp::model::Implementation;

#[derive(Debug, Parser)]
struct Cli {
    #[command(subcommand)]
    command: CliCommand,
}

impl RunnableCommand for Cli {
    fn run(self, app_info: caretta_framework::types::AppInfo) {
        self.command.run(app_info)
    }
}

#[derive(Debug, Subcommand)]
enum CliCommand {
    Dev(DevCommandArgs),
}

impl RunnableCommand for CliCommand {
    fn run(self, app_info: caretta_framework::types::AppInfo) {
        match self {
            CliCommand::Dev(x) => x.run(app_info),
        }
    }
}
fn main() {
    Cli::parse().run(AppInfo {
        name: APP_NAME,
        info: Implementation {
            name: "caretta-framework-example-cli".to_string(),
            title: None,
            version: "0.0.0".to_string(),
            icons: None,
            website_url: None,
        },
    })
}
