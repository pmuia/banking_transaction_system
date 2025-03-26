use sqlx::PgPool;

use crate::repository::transaction_repo::TransactionRepo;
use crate::domain::transaction::Transaction;

pub struct TransactionService;

//ownership of transaction to ensure it’s valid for async execution.
//We borrow pool to avoid taking full ownership of the database connection.
impl TransactionService {
    pub async fn process_transaction(pool: &PgPool, transaction: Transaction) -> Result<(), String> {
        TransactionRepo::create_transaction(pool, &transaction)
            .await
            .map_err(|_| "Failed to save transaction".to_string())?;
        Ok(())
    }

    pub async fn get_all_transactions(pool: &PgPool) -> Result<Vec<Transaction>, sqlx::Error> {
        TransactionRepo::get_all_transactions(pool).await
    }
}
