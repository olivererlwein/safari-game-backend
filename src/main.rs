use anyhow::Context;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use warp::Filter;           // ← necesitas esto para que `use warp::...` compile
use tokio;                  // ← para #[tokio::main]
use dotenvy;                // si usas `dotenvy::dotenv()`

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("../migrations");

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    let database_url = std::env::var("DATABASE_URL")
        .context("DATABASE_URL must be set")?;

    // Conexión y migraciones
    let mut conn = diesel::mysql::MysqlConnection::establish(&database_url)
        .context("Error connecting to database")?;

    conn.run_pending_migrations(MIGRATIONS)
        .context("Error running migrations")?;

    // Aquí montarías tus rutas Warp...
    // let routes = api::progress_routes(...).or(api::world_routes(...));
    // warp::serve(routes).run(([0,0,0,0], 3030)).await;

    Ok(())
}
