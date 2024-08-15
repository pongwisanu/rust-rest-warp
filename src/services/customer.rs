use std::convert::Infallible;
use warp::{self, Filter};

use crate::db::Db;
use crate::handlers;
use crate::models::customer::Customer;

pub fn customer_routes(
    db: Db,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        list_customer(db.clone())
            .or(create_customer(db.clone()))
            .or(get_customer(db.clone()))
            .or(update_customer(db.clone()))
            .or(delete_customer(db.clone()))
}

fn list_customer(db: Db) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("customers")
        .and(warp::get())
        .and(with_db(db))
        .and_then(handlers::customer::list_mails)
}

fn create_customer(db: Db) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("customers")
        .and(warp::post())
        .and(json_body())
        .and(with_db(db))
        .and_then(handlers::customer::create_customer)
}

fn get_customer(db: Db) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("customers" / String)
        .and(warp::get())
        .and(with_db(db))
        .and_then(handlers::customer::get_customer)
}

fn update_customer(db: Db) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("customers" / String)
        .and(warp::put())
        .and(json_body())
        .and(with_db(db))
        .and_then(handlers::customer::update_customer)
}

fn delete_customer(db: Db) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("customers" / String)
        .and(warp::delete())
        .and(with_db(db))
        .and_then(handlers::customer::delete_customer)
}

fn with_db(db: Db) -> impl Filter<Extract = (Db,), Error = Infallible> + Clone {
    warp::any().map(move || db.clone())
}

fn json_body() -> impl Filter<Extract = (Customer,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16)
        .and(warp::body::json())
}
