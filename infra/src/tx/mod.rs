pub(crate) mod ctx;

use async_trait::async_trait;
use sqlx::PgPool;
use std::sync::Arc;

use common::{
    error::AppError,
    tx::{ctx::TransactionContext, BoxFuture, TransactionResult, UnitOfWork},
};
use ctx::PgTxContext;

pub struct PgUnitOfWork {
    pool: Arc<PgPool>,
}

impl PgUnitOfWork {
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UnitOfWork for PgUnitOfWork {
    async fn run_in_transaction(
        &self,
        f: Box<
            dyn for<'a> FnOnce(
                    &'a mut dyn TransactionContext,
                )
                    -> BoxFuture<'a, Result<TransactionResult, AppError>>
                + Send,
        >,
    ) -> Result<TransactionResult, AppError> {
        let tx = self.pool.begin().await.map_err(|err| {
            AppError::internal_with_private("Failed to start transaction!", err.to_string())
        })?;

        let mut ctx = PgTxContext { tx };
        let result = f(&mut ctx).await;

        match result {
            Ok(val) => {
                ctx.tx.commit().await.map_err(|err| {
                    AppError::internal_with_private(
                        "Failed to commit transaction!",
                        err.to_string(),
                    )
                })?;
                Ok(val)
            }
            Err(e) => {
                ctx.tx.rollback().await.map_err(|rollback_err| {
                    AppError::internal_with_private(
                        "Failed to rollback after transaction failure!",
                        rollback_err.to_string(),
                    )
                })?;
                Err(e)
            }
        }
    }
}
