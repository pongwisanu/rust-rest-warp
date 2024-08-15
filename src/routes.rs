use warp::Filter;

use crate::db::Db;
use crate::services;

pub fn routes(db: Db) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone{
    let customer_routes = services::customer::customer_routes(db);

    customer_routes
        .with(warp::log("API"))
}
