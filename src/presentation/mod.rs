use actix_web::web;
pub mod transaction_controller;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api") // All routes are under /api
            .service(transaction_controller::create_transaction)
            .service(transaction_controller::get_all_transactions)
    );
}
