use iroh_docs::{NamespaceId, NamespacePublicKey, api::{Doc, DocsApi}};
use sea_orm::{ActiveValue::{self, Set}, entity::prelude::*};
use serde::{Deserialize, Serialize};
use tracing_subscriber::registry::Data;

use crate::{mcp::{self, model::Error}, types::{Database, WorkspacePublicKey, WorkspaceSecretKey}};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "workspace")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub public_key: WorkspacePublicKey,
    pub name: String
}

impl Model {
    pub async fn from_secret<C>(ctx: &C, secret: WorkspaceSecretKey) -> Result<Self, DbErr>
    where
        C: AsRef<Database>,
    {
        todo!()
    }

    pub async fn new<C>(ctx: &C, name: &str) -> Result<Self, mcp::model::Error>
    where
        C: AsRef<Database> + AsRef<DocsApi>
    {
        let db: &Database = ctx.as_ref();
        let conn: &DatabaseConnection = db.as_ref();
        let docs: &DocsApi = ctx.as_ref();
        let doc = docs.create().await.map_err(|e| mcp::model::Error::Docs(e.to_string()))?;
        let public_key = WorkspacePublicKey::from(doc.id().into_public_key().unwrap());
        ActiveModel {
            public_key: ActiveValue::Set(public_key), 
            name: ActiveValue::Set(name.to_string()),
            ..Default::default()
        }.insert(conn).await.map_err(|e| Error::Docs(e.to_string()))
    }

    pub async fn get_by_id<C>(ctx: &C, id: i64) -> Result<Option<Self>, DbErr>
    where
        C: AsRef<Database>,
    {
        let db = AsRef::<DatabaseConnection>::as_ref(ctx.as_ref());
        Entity::find_by_id(id).one(db).await
    }

    pub async fn get_by_name<C>(ctx: &C, name: &str) -> Result<Option<Self>, DbErr>
    where
        C: AsRef<Database>,
    {
        let db = AsRef::<DatabaseConnection>::as_ref(ctx.as_ref());
        Entity::find().filter(Column::Name.eq(name)).one(db).await
    }

    pub async fn get_doc<C>(&self, ctx:&C) -> Result<Doc, Error> 
    where 
        C: AsRef<DocsApi>
    {
        let docs: &DocsApi = ctx.as_ref();
        let namespace_public= NamespacePublicKey::from(self.public_key.clone());
        let namespace_id = NamespaceId::from(namespace_public);
        match docs.open(namespace_id).await {
            Ok(Some(x)) => Ok(x),
            Ok(None) => {
                tracing::error!("Failed to get doc");
                Err(Error::Docs(format!("Failed to get doc {:?} from DocsAPI", self)))
            },
            Err(e) => {
                Err(Error::Docs(e.to_string()))
            }
        }
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
        let db = crate::tests::service_context().await;
        let model = Model::new(db, "test").await.unwrap();
        assert_eq!(model, Model::get_by_name(db, "test").await.unwrap().unwrap());
    }
}
