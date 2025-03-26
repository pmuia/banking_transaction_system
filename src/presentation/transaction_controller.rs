use actix_web::{post, get, web, HttpResponse, Responder};
use crate::application::transaction_service::TransactionService;
use crate::domain::transaction::Transaction;

use sqlx::PgPool;

#[post("/transaction")]
async fn create_transaction(pool: web::Data<PgPool>, transaction: web::Json<Transaction>) -> impl Responder {
    match TransactionService::process_transaction(pool.get_ref(), transaction.into_inner()).await {
        Ok(_) => HttpResponse::Ok().json("Transaction processed successfully"),
        Err(err) => HttpResponse::InternalServerError().json(err),
    }
}

#[get("/transactions")]
async fn get_all_transactions(pool: web::Data<PgPool>) -> impl Responder {
    let transactions = TransactionService::get_all_transactions(pool.get_ref()).await;
    match transactions {
        Ok(data) => HttpResponse::Ok().json(data),
        Err(_) => HttpResponse::InternalServerError().json("Failed to fetch transactions"),
    }
}
