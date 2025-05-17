pub trait TransactionContext: Send {
    /// Generic method for type identification without leaking database-specific logic.
    fn type_id(&self) -> std::any::TypeId;
}
