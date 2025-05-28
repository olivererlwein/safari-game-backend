use warp::{Reply, Rejection};
use serde::{Deserialize, Serialize};
use crate::db::{Database, models::PlayerProgress};

#[derive(Debug, Deserialize, Serialize)]
pub struct ProgressRequest {
    pub player_id: String,
    pub world_id: u8,
    pub level: u8,
    pub score: u16,
}

pub fn routes(db: Database) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("api" / "progress")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_db(db))
        .and_then(handle_save_progress)
}

async fn handle_save_progress(
    data: ProgressRequest,
    db: Database
) -> Result<impl Reply, Rejection> {
    let progress = PlayerProgress {
        progress_id: None,
        player_id: data.player_id,
        world_id: data.world_id as i8,
        current_level: data.level as i8,
        levels_completed: data.level as i8, // Ajustar según tu lógica de juego
        quiz_score: data.score as i16,
        last_played: None,
    };
    
    db.save_progress(&progress.player_id, &progress)
        .map_err(|e| warp::reject::custom(ApiError::from(e)))?;
    
    Ok(warp::reply::json(&"Progress saved"))
}

// Helper para inyectar la DB
fn with_db(db: Database) -> impl Filter<Extract = (Database,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db.clone())
}