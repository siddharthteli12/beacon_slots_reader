use actix_web::{get, middleware::Logger, App, HttpResponse, HttpServer, Responder};

/// Returns latest finalized block.
#[get("/current_block")]
async fn current_block() -> impl Responder {
    /// Call to service for current block.
    HttpResponse::Ok().body("Current block is ")
}

/// Attestation rate for given epoch number.
#[get("/attestation_percentage")]
async fn attestation_percentage(_epoch_num: String) -> impl Responder {
    /// Call to service for attestation percentage.
    HttpResponse::Ok().body("Attestation percentage is ")
}

/// Participation rate of a given validator in attestation call.
#[get("/validator_participation_rate")]
async fn validator_participation_rate(_validator_account_id: String) -> impl Responder {
    /// Call to service for attestation percentage.
    HttpResponse::Ok().body("Attestation percentage is ")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::new("%U"))
            .service(current_block)
            .service(attestation_percentage)
            .service(validator_participation_rate)
    })
    .bind(("127.0.0.1", 8082))?
    .run()
    .await
}
