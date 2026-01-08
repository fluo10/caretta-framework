use std::marker::PhantomData;

use crate::{args::subcommand::ConfigCommandArgs, config::ServerConfig, mcp::ServiceContext, types::AppInfo, util::RunnableCommand};
use clap::{Parser, Subcommand};
use rmcp::{RoleServer, Service};
use sea_orm_migration::MigratorTrait;

use crate::{args::option::ConfigOptionArgs, config::parsed::ParsedConfig, types::Verbosity};

#[derive(Parser, Debug)]
pub struct ServerParser<S, M>
where
    S: Service<RoleServer> + From<&'static ServiceContext> + Send + 'static,
    M: MigratorTrait,
{
    #[arg(skip)]
    server: PhantomData<S>,
    #[arg(skip)]
    migrator: PhantomData<M>,
    #[command(flatten)]
    config: ConfigOptionArgs,
    #[command(subcommand)]
    command: Option<ServerSubcommand>
}

impl<S, M> ServerParser<S, M>
where
    S: Service<RoleServer> + From<&'static ServiceContext> + Send + 'static,
    M: MigratorTrait,
{
    #[tokio::main]
    async fn spawn_server(self, app_info: AppInfo) {
        let app_name = app_info.name;
        let mut check_config: bool;
        let mut verbosity: Verbosity;
        let config_to_print: Option<ParsedConfig> = None;
        let mut config = self.config.into_parsed_config(app_name)
            .with_default(app_name)
            .with_database().await;
        
        let config = config.into_server_config(app_name).unwrap();
        config.log.init_tracing_subscriber(true);
        config.spawn_server::<S, M>(app_name).await
    }
}


impl<S, M> RunnableCommand for ServerParser<S, M>
where
    S: Service<RoleServer> + From<&'static ServiceContext> + Send + 'static,
    M: MigratorTrait,
{
    fn run(self, app_info: AppInfo) {
        if let Some(x) = self.command {
            x.run(app_info)
        } else {
            self.spawn_server(app_info);
        }
    }
}


#[derive(Debug, Subcommand)]
enum ServerSubcommand {
    Config(ConfigCommandArgs)
}

impl RunnableCommand for ServerSubcommand {
    fn run(self, app_info: AppInfo) {
        match self {
            ServerSubcommand::Config(x) => x.run(app_info)
        }
    }
}