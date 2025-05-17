use sqlx::{Postgres, Transaction};
use std::any::TypeId;

use common::tx::ctx::TransactionContext;

pub struct PgTxContext<'a> {
    pub tx: Transaction<'a, Postgres>,
}

impl<'a> TransactionContext for PgTxContext<'a> {
    fn type_id(&self) -> TypeId {
        TypeId::of::<PgTxContext<'static>>()
    }
}

pub trait PgTransactionContextExt {
    fn as_postgres_tx(&mut self) -> Option<&mut Transaction<'_, Postgres>>;
}

impl<'a> PgTransactionContextExt for dyn TransactionContext + 'a {
    fn as_postgres_tx(&mut self) -> Option<&mut Transaction<'_, Postgres>> {
        if self.type_id() == TypeId::of::<PgTxContext<'static>>() {
            unsafe {
                let pg_ctx = &mut *(self as *mut dyn TransactionContext as *mut PgTxContext);
                Some(&mut pg_ctx.tx)
            }
        } else {
            None
        }
    }
}
