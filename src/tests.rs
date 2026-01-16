use std::{
    marker::PhantomData,
    path::PathBuf,
    sync::{Arc, LazyLock},
};

use crate::{
    config::{LogConfig, P2pConfig, ServerConfig, ServerConfigExt as _, StorageConfig, parsed::{ParsedConfig, ParsedLogConfig, ParsedMcpConfig, ParsedP2pConfig, ParsedStorageConfig}}, entity::config, mcp::ServiceContext, types::Database, util::Emptiable
};
use caretta_framework_migration::Migrator;
use iroh::Endpoint;
use sea_orm::DatabaseConnection;
use tokio::sync::OnceCell;
use tracing::Level;

pub static APP_NAME: &str = "caretta-framework-test";

static PARSED_CONFIG: OnceCell<ParsedConfig> = OnceCell::const_new();
pub async fn parsed_config() -> &'static ParsedConfig {
    PARSED_CONFIG.get_or_init(|| async {
        let dir = tempfile::Builder::new()
            .prefix("caretta_brain_test")
            .tempdir()
            .unwrap()
            .keep();
        let data_dir = Some(dir.join("data"));
        let cache_dir = Some(dir.join("cache"));
        let storage = ParsedStorageConfig {
            data_dir,
            cache_dir,
        };

        ParsedConfig{
            storage: storage,
            mcp: ParsedMcpConfig::empty(),
            p2p: ParsedP2pConfig::empty(),
            log: ParsedLogConfig::empty(),
        }.with_default(APP_NAME).with_database().await
    }).await
}

static SERVER_CONFIG: OnceCell<ServerConfig> = OnceCell::const_new();

pub async fn server_config() -> &'static ServerConfig {
    SERVER_CONFIG.get_or_init(|| async {
        parsed_config().await.clone().into_server_config(APP_NAME).unwrap()
    }).await
}

static SERVICE_CONTEXT: OnceCell<ServiceContext> = OnceCell::const_new();

pub async fn service_context() -> &'static ServiceContext {
    SERVICE_CONTEXT.get_or_init(|| async move {
        let server_config = server_config().await;
                let mcp_config = &server_config.mcp;
        let p2p_config = &server_config.p2p;
        let storage_config = &server_config.storage;
        let (iroh_endpoint, iroh_docs, iroh_router_builder) =
            server_config.to_iroh_router_builder(APP_NAME).await.unwrap();
        let database = storage_config.open_database().await;
        let ct = tokio_util::sync::CancellationToken::new();
        ServiceContext {
                app_database: storage_config.open_app_database::<caretta_framework_migration::Migrator>(APP_NAME).await,
                database: database,
                iroh_endpoint: iroh_endpoint,
                docs: iroh_docs.api().clone(),
        }
    }).await
}

