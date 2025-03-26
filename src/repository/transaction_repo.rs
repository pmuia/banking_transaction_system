use redis::AsyncCommands;
use sqlx::PgPool;
use crate::domain::transaction::Transaction;
use sqlx::Error;
use log::{info, error};

pub struct TransactionRepo;

impl TransactionRepo {
    /// Fetch all transactions, using Redis cache if available.
    pub async fn get_all_transactions(pool: &PgPool, redis_client: &redis::Client) -> Result<Vec<Transaction>, Error> {
        let mut con = redis_client.get_async_connection().await.map_err(|err| {
            error!("Failed to get Redis connection: {}", err);
            err
        })?;

        // Check Redis cache
        if let Ok(Some(cached_data)) = con.get::<_, Option<String>>("transactions_cache").await {
            info!("Returning cached transactions");
            return Ok(serde_json::from_str(&cached_data).unwrap());
        }

        // Fetch from DB
        let transactions = sqlx::query_as!(
            Transaction,
            "SELECT id, sender_id, receiver_id, amount, created_at FROM transactions"
        )
        .fetch_all(pool)
        .await?;

        // Cache result in Redis
        let _ = con.set_ex("transactions_cache", serde_json::to_string(&transactions).unwrap(), 60).await;

        Ok(transactions)
    }

    /// Create a new transaction and invalidate Redis cache.
    pub async fn create_transaction(
        pool: &PgPool,
        redis_client: &redis::Client,
        sender_id: i32,
        receiver_id: i32,
        amount: f64
    ) -> Result<Transaction, Error> {
        let transaction = sqlx::query_as!(
            Transaction,
            "INSERT INTO transactions (sender_id, receiver_id, amount) VALUES ($1, $2, $3) RETURNING id, sender_id, receiver_id, amount, created_at",
            sender_id,
            receiver_id,
            amount
        )
        .fetch_one(pool)
        .await?;

        info!("Transaction created successfully: {:?}", transaction);

        // Invalidate Redis cache
        let mut con = redis_client.get_async_connection().await.map_err(|err| {
            error!("Failed to get Redis connection: {}", err);
            err
        })?;

        if let Err(err) = con.del("transactions_cache").await {
            error!("Failed to invalidate Redis cache: {}", err);
        }

        Ok(transaction)
    }
}
