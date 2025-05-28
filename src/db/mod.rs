use diesel::{prelude::*, mysql::MysqlConnection};
use diesel::result::Error;
use crate::models::{PlayerProgress, World};

pub struct Database {
    pub conn: MysqlConnection,
}

impl Database {
    pub fn new(database_url: &str) -> Self {
        let conn = MysqlConnection::establish(database_url)
            .expect("Error connecting to MySQL");
        Self { conn }
    }

    pub fn save_progress(
        &self,
        player_id: &str,
        progress: &PlayerProgress
    ) -> Result<(), Error> {
        use crate::schema::player_progress::dsl::*;
        
        diesel::insert_into(player_progress)
            .values(progress)
            .on_conflict((player_id, world_id))
            .do_update()
            .set((
                current_level.eq(diesel::dsl::greatest(current_level, progress.current_level)),
                levels_completed.eq(diesel::dsl::greatest(levels_completed, progress.levels_completed)),
                quiz_score.eq(quiz_score + progress.quiz_score),
                last_played.eq(diesel::dsl::now),
            ))
            .execute(&self.conn)?;
            
        Ok(())
    }

    pub fn get_worlds(&self) -> Result<Vec<World>, Error> {
        use crate::schema::worlds::dsl::*;
        worlds.load::<World>(&self.conn)
    }

    pub fn get_player_progress(&self, player_id: &str) -> Result<Vec<PlayerProgress>, Error> {
        use crate::schema::player_progress::dsl::*;
        player_progress.filter(player_id.eq(player_id))
            .load::<PlayerProgress>(&self.conn)
    }
}