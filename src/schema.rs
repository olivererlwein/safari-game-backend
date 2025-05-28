// backend/src/schema.rs

table! {
    players (player_id) {
        player_id -> Varchar,
        player_name -> Varchar,
        created_at -> Timestamp,
        last_active -> Timestamp,
    }
}

table! {
    worlds (world_id) {
        world_id -> TinyInt,
        world_name -> Varchar,
    }
}

table! {
    player_progress (progress_id) {
        progress_id -> Integer,
        player_id -> Varchar,
        world_id -> TinyInt,
        current_level -> TinyInt,
        levels_completed -> TinyInt,
        quiz_score -> SmallInt,
        last_played -> Nullable<Timestamp>,
    }
}