//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "submission")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub name: String,
    pub desription: Option<String>,
    pub asigned_to: String,
    pub create_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::reminder::Entity")]
    Reminder,
}

impl Related<super::reminder::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Reminder.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
