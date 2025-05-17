use async_trait::async_trait;
use sqlx::PgPool;
use std::sync::Arc;
use uuid::Uuid;

use crate::{db::note as db, tx::ctx::PgTransactionContextExt};
use common::{error::AppError, params::PaginationParams, tx::ctx::TransactionContext};
use dmn::{
    entities::note::{CreateNoteData, Note, NoteData, UpdateNoteData},
    repos::note::{GetNotesResponse, NoteRepository},
};

pub struct PgNoteRepository {
    db_pool: Arc<PgPool>,
}

impl PgNoteRepository {
    pub fn new(db_pool: Arc<PgPool>) -> Self {
        Self { db_pool }
    }
}

#[async_trait]
impl NoteRepository for PgNoteRepository {
    async fn get_note(
        &self,
        ctx: Option<&mut dyn TransactionContext>,
        note_id: Uuid,
        user_id: Option<Uuid>,
    ) -> Result<Note, AppError> {
        let db_note = match ctx {
            Some(ctx) => {
                let pg_tx = ctx.as_postgres_tx().ok_or_else(|| {
                    AppError::internal("Invalid transaction context for Postgres repository")
                })?;
                db::get_note(&mut **pg_tx, note_id, user_id).await?
            }
            None => db::get_note(&*self.db_pool, note_id, user_id).await?,
        };
        let note_data: NoteData = db_note.into();
        let note = note_data.try_into()?;
        Ok(note)
    }

    async fn get_notes(
        &self,
        ctx: Option<&mut dyn TransactionContext>,
        pagination: &PaginationParams,
        user_id: Option<Uuid>,
    ) -> Result<GetNotesResponse, AppError> {
        let (db_notes, count) = match ctx {
            Some(ctx) => {
                let pg_tx = ctx.as_postgres_tx().ok_or_else(|| {
                    AppError::internal("Invalid transaction context for Postgres repository")
                })?;
                let db_notes = db::get_notes(&mut **pg_tx, pagination, user_id).await?;
                let count = db::get_note_count(&mut **pg_tx, user_id).await?;
                (db_notes, count)
            }
            None => {
                let db_notes = db::get_notes(&*self.db_pool, pagination, user_id).await?;
                let count = db::get_note_count(&*self.db_pool, user_id).await?;
                (db_notes, count)
            }
        };
        let notes = db_notes
            .into_iter()
            .map(|db_note| db_note.into())
            .map(|note_data: NoteData| note_data.try_into())
            .collect::<Result<_, AppError>>()?;
        let dmn_res = GetNotesResponse { notes, count };
        Ok(dmn_res)
    }

    async fn create_note(
        &self,
        ctx: Option<&mut dyn TransactionContext>,
        note_data: CreateNoteData,
    ) -> Result<Note, AppError> {
        let db_note = match ctx {
            Some(ctx) => {
                let pg_tx = ctx.as_postgres_tx().ok_or_else(|| {
                    AppError::internal("Invalid transaction context for Postgres repository")
                })?;
                db::create_note(&mut **pg_tx, note_data.into()).await?
            }
            None => db::create_note(&*self.db_pool, note_data.into()).await?,
        };
        let note_data: NoteData = db_note.into();
        let note = note_data.try_into()?;
        Ok(note)
    }

    async fn update_note(
        &self,
        ctx: Option<&mut dyn TransactionContext>,
        note_id: Uuid,
        note_data: UpdateNoteData,
        user_id: Option<Uuid>,
    ) -> Result<Note, AppError> {
        let db_note = match ctx {
            Some(ctx) => {
                let pg_tx = ctx.as_postgres_tx().ok_or_else(|| {
                    AppError::internal("Invalid transaction context for Postgres repository")
                })?;
                db::update_note(&mut **pg_tx, note_id, note_data.into(), user_id).await?
            }
            None => db::update_note(&*self.db_pool, note_id, note_data.into(), user_id).await?,
        };
        let note_data: NoteData = db_note.into();
        let note = note_data.try_into()?;
        Ok(note)
    }

    async fn delete_note(
        &self,
        ctx: Option<&mut dyn TransactionContext>,
        note_id: Uuid,
        user_id: Option<Uuid>,
    ) -> Result<(), AppError> {
        match ctx {
            Some(ctx) => {
                let pg_tx = ctx.as_postgres_tx().ok_or_else(|| {
                    AppError::internal("Invalid transaction context for Postgres repository")
                })?;
                db::delete_note(&mut **pg_tx, note_id, user_id).await?
            }
            None => db::delete_note(&*self.db_pool, note_id, user_id).await?,
        };
        Ok(())
    }
}

impl From<CreateNoteData> for db::CreateNoteDataPg {
    fn from(dmn_note_data: CreateNoteData) -> Self {
        Self {
            user_id: dmn_note_data.user_id,
            title: dmn_note_data.title,
            content: dmn_note_data.content,
        }
    }
}

impl From<UpdateNoteData> for db::UpdateNoteDataPg {
    fn from(dmn_note_data: UpdateNoteData) -> Self {
        Self {
            title: dmn_note_data.title,
            content: dmn_note_data.content,
        }
    }
}
