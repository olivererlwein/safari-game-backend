pub mod progress;
pub mod world;

use warp::Filter;
use crate::db::Database;

pub fn progress_routes(db: Database) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    progress::routes(db)
}

pub fn world_routes(db: Database) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    world::routes(db)
}