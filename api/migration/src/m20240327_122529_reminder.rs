use sea_orm_migration::prelude::*;

use crate::m20240327_122519_submission::Submission;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Reminder::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Reminder::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Reminder::Title).string().not_null())
                    .col(ColumnDef::new(Reminder::RemindAt).date_time().not_null())
                    .col(ColumnDef::new(Reminder::IsReminded).boolean().not_null())
                    .col(ColumnDef::new(Reminder::SubmissionId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .from(Reminder::Table, Reminder::SubmissionId)
                            .to(Submission::Table, Submission::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Reminder::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Reminder {
    Table,
    Id,
    Title,
    RemindAt,
    IsReminded,
    SubmissionId,
}
