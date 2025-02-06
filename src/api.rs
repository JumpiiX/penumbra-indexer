use std::sync::Arc;
use warp::{Filter, Rejection, Reply};
use crate::db::Database;

pub struct Api {
    db: Arc<Database>,
}

impl Api {
    pub fn new(db: Database) -> Self {
        Self {
            db: Arc::new(db),
        }
    }

    pub fn routes(&self) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
        let db = self.db.clone();

        warp::path("blocks")
            .and(warp::get())
            .and(with_db(db))
            .and_then(handle_get_blocks)
    }
}

fn with_db(
    db: Arc<Database>,
) -> impl Filter<Extract = (Arc<Database>,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db.clone())
}

async fn handle_get_blocks(db: Arc<Database>) -> Result<impl Reply, Rejection> {
    match db.get_latest_blocks(10).await {
        Ok(blocks) => Ok(warp::reply::json(&blocks)),
        Err(_) => Err(warp::reject::custom(ApiError)),
    }
}

#[derive(Debug)]
struct ApiError;
impl warp::reject::Reject for ApiError {}