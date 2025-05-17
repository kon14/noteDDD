pub mod ctx;

use async_trait::async_trait;
use std::{any::Any, future::Future, pin::Pin};

use crate::error::AppError;
use ctx::TransactionContext;

pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = T> + Send + 'a>>;

pub struct TransactionResult {
    inner: Box<dyn Any + Send>,
}

impl TransactionResult {
    pub fn new<T: Send + 'static>(value: T) -> Self {
        Self {
            inner: Box::new(value),
        }
    }

    pub fn extract<T: 'static>(self) -> Result<T, AppError> {
        self.inner
            .downcast::<T>()
            .map(|boxed| *boxed)
            .map_err(|_| AppError::internal("Failed to extract transaction result"))
    }
}

#[async_trait]
pub trait UnitOfWork: Send + Sync {
    async fn run_in_transaction(
        &self,
        f: Box<
            dyn for<'a> FnOnce(
                    &'a mut dyn TransactionContext,
                )
                    -> BoxFuture<'a, Result<TransactionResult, AppError>>
                + Send,
        >,
    ) -> Result<TransactionResult, AppError>;
}
