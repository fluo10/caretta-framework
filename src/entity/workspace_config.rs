use sea_orm::{ActiveValue::Set, entity::prelude::*};
use tracing_subscriber::registry::Data;

use crate::types::{Database, WorkspacePublicKey, WorkspaceSecretKey};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "workspace_config")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i8,
    pub secret_key: WorkspaceSecretKey,
    pub name: String
}

impl Model {
    const ID: i8 = 0;
    pub async fn from_secret<C>(ctx: &C, secret: WorkspaceSecretKey) -> Result<Self, DbErr>
    where
        C: AsRef<Database>,
    {
        todo!()
    }

    pub async fn new<C>(ctx: &C) -> Result<Self, DbErr>
    where
        C: AsRef<Database>,
    {
        todo!()
    }

    pub async fn get<C>(ctx: &C) -> Result<Option<Self>, DbErr>
    where
        C: AsRef<Database>,
    {
        let db = AsRef::<DatabaseConnection>::as_ref(ctx.as_ref());
        Entity::find_by_id(Self::ID).one(db).await
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn insert_and_get_record() {
        let db = &crate::tests::service_context().await.database;
        let model = Model::new(db).await.unwrap();
        assert_eq!(model, Model::get(db).await.unwrap().unwrap());
    }
}
