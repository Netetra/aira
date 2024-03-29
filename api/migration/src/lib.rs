pub use sea_orm_migration::prelude::*;

mod m20240327_122519_submission;
mod m20240327_122529_reminder;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20240327_122519_submission::Migration),
            Box::new(m20240327_122529_reminder::Migration),
        ]
    }
}
