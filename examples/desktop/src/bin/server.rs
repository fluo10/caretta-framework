use caretta_framework::{args::parser::ServerParser, types::AppInfo, util::RunnableCommand as _};
use caretta_framework_example_core::APP_NAME;
use clap::Parser;
use rmcp::model::Implementation;

fn main() {
    let parser = ServerParser::<
        caretta_framework_example_core::mcp::Service,
        caretta_framework_example_core::migration::Migrator,
    >::parse();
    parser.run(AppInfo {
        name: APP_NAME,
        info: Implementation {
            name: "caretta-framework-example-server".to_string(),
            title: None,
            version: "0.0.0".to_string(),
            icons: None,
            website_url: None,
        },
    })
}
