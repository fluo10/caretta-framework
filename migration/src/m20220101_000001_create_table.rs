use sea_orm_migration::{prelude::*, sea_orm::DatabaseBackend};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        match manager.get_database_backend() {
            DatabaseBackend::Sqlite => {
                let db = manager.get_connection();

                db.execute_unprepared(
                    "CREATE TABLE config (
                        id              INTEGER PRIMARY KEY CHECK (id = 0),
                        p2p_secret_key  BLOB NOT NULL,
                        p2p_enable_n0   BOOL NOT NULL,
                        p2p_enable_mdns BOOL NOT NULL
                    )",
                )
                .await?;
                db.execute_unprepared(
                    "CREATE TABLE workspace (
                        id         INTEGER PRIMARY KEY,
                        public_key BLOB NOT NULL,
                        name       STRING NOT NULL
                    )",
                )
                .await?;
                Ok(())
            }
            x => Err(DbErr::Migration(format!(
                "Expected Sqlite, found {}",
                x.as_str()
            ))),
        }
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        unimplemented!();
    }
}
