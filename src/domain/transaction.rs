use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};  // Import serde traits

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub id: i32,
    pub sender_id: i32,
    pub receiver_id: i32,
    pub amount: f64,
    pub timestamp: NaiveDateTime,
}