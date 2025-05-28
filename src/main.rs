mod api;
mod db;
mod models;
mod schema;

use warp::Filter;
use crate::db::Database;

#[tokio::main]
async fn main() {
    // Configuración inicial
    dotenv::dotenv().ok();
    let db = Database::new(&std::env::var("DATABASE_URL").expect("DATABASE_URL must be set"));

    // Rutas API
    let api_routes = api::progress_routes(db.clone())
        .or(api::world_routes(db))
        .with(warp::cors().allow_any_origin());

    // Servir frontend WASM
    let static_files = warp::path::end()
        .and(warp::fs::file("./frontend/dist/index.html"));

    warp::serve(api_routes.or(static_files))
        .run(([0, 0, 0, 0], 3030))
        .await;
}