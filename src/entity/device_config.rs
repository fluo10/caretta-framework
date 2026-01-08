use sea_orm::{ActiveValue::Set, entity::prelude::*};

use crate::{
    config::P2pConfig,
    types::{Database, DeviceSecretKey},
};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "device_config")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i8,
    pub p2p_secret_key: DeviceSecretKey,
    pub p2p_enable_n0: bool,
    pub p2p_enable_mdns: bool,
}

impl Model {
    const ID: i8 = 0;

    pub async fn get_or_try_init<T>(ctx: &T) -> Result<Self, DbErr> 
    where 
        T: AsRef<Database>
    {
        let db = AsRef::<DatabaseConnection>::as_ref(ctx.as_ref());
        if let Some(x) = Entity::find_by_id(Self::ID).one(db).await? {
            Ok(x)
        } else {
            Ok(ActiveModel {
                id: Set(Self::ID),
                p2p_secret_key: Set(DeviceSecretKey::new()),
                p2p_enable_n0: Set(true),
                p2p_enable_mdns: Set(true),
            }
            .insert(db)
            .await?)
        }
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl From<Model> for P2pConfig {
    fn from(value: Model) -> Self {
        P2pConfig {
            secret_key: value.p2p_secret_key,
            enable_mdns: value.p2p_enable_mdns,
            enable_n0: value.p2p_enable_n0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn insert_and_get_record() {
        let db = crate::tests::database().await;
        let model = Model::get_or_try_init(db).await.unwrap();
        assert_eq!(model, Model::get_or_try_init(db).await.unwrap());
    }
}
