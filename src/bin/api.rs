use actix_web::{get, middleware::Logger, web, App, HttpResponse, HttpServer, Responder};
use dotenv::dotenv;
use indexer::{calculation, db::get_slot_participation};
use serde::Serialize;

#[derive(Serialize)]
struct Response {
    start: i64,
    average_participation: f64,
    end: i64,
}
/// Returns latest finalized block.
#[get("/current_block")]
async fn current_block() -> impl Responder {
    // Call to service for current block.
    HttpResponse::Ok().body("Current block is ")
}

/// Attestation rate for given epoch number.
#[get("/attestation_percentage")]
async fn attestation_percentage() -> impl Responder {
    dotenv().ok();
    let db_url = std::env::var("POSTGRES_URL").expect("ENV VARIABLE must be set.");
    let db_pool = sqlx::postgres::PgPool::connect(&db_url)
        .await
        .expect("Issue connecting with db");

    let slot_participation = get_slot_participation(db_pool).await.unwrap();
    let result = calculation::calculate(slot_participation);
    let res = Response {
        start: result.0,
        average_participation: result.2,
        end: result.1,
    };
    web::Json(res)
}

/// Participation rate of a given validator in attestation call.
#[get("/validator_participation_rate")]
async fn validator_participation_rate(_validator_account_id: String) -> impl Responder {
    // Call to service for current block.
    HttpResponse::Ok().body("Validator participation is ")
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
