use warp::{Reply, Rejection};
use crate::db::Database;

pub fn routes(db: Database) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
    warp::path!("api" / "worlds")
        .and(warp::get())
        .and(with_db(db))
        .and_then(get_worlds_handler)
}

async fn get_worlds_handler(db: Database) -> Result<impl Reply, Rejection> {
    let worlds = db.get_worlds()
        .map_err(|e| warp::reject::custom(ApiError::from(e)))?;
    Ok(warp::reply::json(&worlds))
}