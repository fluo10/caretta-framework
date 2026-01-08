use crate::{args::option::ClientOptionArgs, mcp::Api, types::AppInfo, util::RunnableCommand};
use clap::Args;

use crate::args::option::DeviceIdentifierOptionArgs;

#[derive(Debug, Args)]
pub struct DevPingCommandArgs {
    #[command(flatten)]
    target: DeviceIdentifierOptionArgs,
    #[command(flatten)]
    client: ClientOptionArgs,

}

impl RunnableCommand for DevPingCommandArgs {
    #[tokio::main]
    async fn run(self, app_info: AppInfo) {
        let client = self.client.init_tracing_subscriber_and_spawn_client(app_info).await;
        let response = client
            .dev_ping(crate::mcp::model::DevPingRequest {
                target: self.target.into(),
            })
            .await
            .unwrap();
        println!("rtt: {:?}", response.rtt)
    }
}
