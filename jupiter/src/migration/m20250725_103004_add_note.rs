use sea_orm_migration::prelude::*;

// Step 1: Define an enum to represent table names and all column names#[derive(Iden)]
#[derive(Iden)]
enum Notes {
    Table,
    Id,
    PublicId,
    CommentsCount,
    DiscardedAt,
    OrganizationMembershipId,
    DescriptionHtml,
    DescriptionState,
    DescriptionSchemaVersion,
    Title,
    CreatedAt,
    UpdatedAt,
    OriginalProjectId,
    OriginalPostId,
    OriginalDigestId,
    Visibility,
    NonMemberViewsCount,
    ResolvedCommentsCount,
    ProjectId,
    LastActivityAt,
    ContentUpdatedAt,
    ProjectPermission,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Step 1: Create the table (excluding indexes)
        manager
            .create_table(
                Table::create()
                    .table(Notes::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Notes::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Notes::PublicId).string_len(12).not_null())
                    .col(
                        ColumnDef::new(Notes::CommentsCount)
                            .unsigned()
                            .not_null()
                            .default(0),
                    )
                    .col(ColumnDef::new(Notes::DiscardedAt).date_time())
                    .col(
                        ColumnDef::new(Notes::OrganizationMembershipId)
                            .big_unsigned()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Notes::DescriptionHtml).text())
                    .col(ColumnDef::new(Notes::DescriptionState).text())
                    .col(
                        ColumnDef::new(Notes::DescriptionSchemaVersion)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(ColumnDef::new(Notes::Title).text())
                    .col(ColumnDef::new(Notes::CreatedAt).date_time().not_null())
                    .col(ColumnDef::new(Notes::UpdatedAt).date_time().not_null())
                    .col(ColumnDef::new(Notes::OriginalProjectId).big_unsigned())
                    .col(ColumnDef::new(Notes::OriginalPostId).big_unsigned())
                    .col(ColumnDef::new(Notes::OriginalDigestId).big_unsigned())
                    .col(ColumnDef::new(Notes::Visibility).integer().not_null().default(0))
                    .col(
                        ColumnDef::new(Notes::NonMemberViewsCount)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .col(ColumnDef::new(Notes::ResolvedCommentsCount).integer().default(0_i32))
                    .col(ColumnDef::new(Notes::ProjectId).big_unsigned())
                    .col(ColumnDef::new(Notes::LastActivityAt).date_time())
                    .col(ColumnDef::new(Notes::ContentUpdatedAt).date_time())
                    .col(
                        ColumnDef::new(Notes::ProjectPermission)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .to_owned(),
            )
            .await?;

        // Step 2: Create indexes separately
        manager
            .create_index(
                Index::create()
                    .unique()
                    .name("index_notes_on_public_id")
                    .table(Notes::Table)
                    .col(Notes::PublicId)
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("index_notes_on_content_updated_at")
                    .table(Notes::Table)
                    .col(Notes::ContentUpdatedAt)
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("index_notes_on_created_at")
                    .table(Notes::Table)
                    .col(Notes::CreatedAt)
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("index_notes_on_discarded_at")
                    .table(Notes::Table)
                    .col(Notes::DiscardedAt)
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("index_notes_on_last_activity_at")
                    .table(Notes::Table)
                    .col(Notes::LastActivityAt)
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("index_notes_on_organization_membership_id")
                    .table(Notes::Table)
                    .col(Notes::OrganizationMembershipId)
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("index_notes_on_project_id")
                    .table(Notes::Table)
                    .col(Notes::ProjectId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Notes::Table).to_owned())
            .await
    }
}
