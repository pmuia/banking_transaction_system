use actix_web::{post, web, HttpResponse, Responder};
use crate::application::TransactionService;
use sqlx::PgPool;

#[post("/transaction")]
async fn process_transaction(pool: web::Data<PgPool>, transaction: web::Json<Transaction>) -> impl Responder {
    match TransactionService::process_transaction(pool.get_ref(), transaction.into_inner()).await {
        Ok(_) => HttpResponse::Ok().json("Transaction successful"),
        Err(err) => HttpResponse::InternalServerError().json(err),
    }
}
