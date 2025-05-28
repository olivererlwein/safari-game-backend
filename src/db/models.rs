// backend/src/db/models.rs
use crate::schema::*;
use serde::{Serialize, Deserialize};

#[derive(Queryable, Insertable, Serialize, Deserialize)]
#[diesel(table_name = players)]
pub struct Player {
    pub player_id: String,
    pub player_name: String,
    pub created_at: chrono::NaiveDateTime,
    pub last_active: chrono::NaiveDateTime,
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct World {
    pub world_id: i8,
    pub world_name: String,
}

#[derive(Queryable, Insertable, AsChangeset, Serialize, Deserialize)]
#[diesel(table_name = player_progress)]
pub struct PlayerProgress {
    pub progress_id: Option<i32>,  // Auto-increment
    pub player_id: String,
    pub world_id: i8,
    pub current_level: i8,
    pub levels_completed: i8,
    pub quiz_score: i16,
    pub last_played: Option<chrono::NaiveDateTime>,
}