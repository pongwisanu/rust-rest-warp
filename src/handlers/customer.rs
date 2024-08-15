use std::convert::Infallible;
use warp::{self, http::StatusCode};

use crate::db::Db;
use crate::models::customer::Customer;

pub async fn list_mails(db: Db) -> Result<impl warp::Reply, Infallible> {
    let customers = db.lock().await;
    let customers: Vec<Customer> = customers.clone();
    Ok(warp::reply::json(&customers))
}

pub async fn create_customer(
    new_customer: Customer,
    db: Db,
) -> Result<impl warp::Reply, Infallible> {
    let mut customers = db.lock().await;

    for customer in customers.iter() {
        if customer.id == new_customer.id {
            return Ok(StatusCode::BAD_REQUEST);
        }
    }

    customers.push(new_customer);

    Ok(StatusCode::CREATED)
}

pub async fn get_customer(id: String, db: Db) -> Result<Box<dyn warp::Reply>, Infallible> {
    let customers = db.lock().await;

    for customer in customers.iter() {
        if customer.id == id {
            return Ok(Box::new(warp::reply::json(&customer)));
        }
    }

    Ok(Box::new(StatusCode::NOT_FOUND))
}

pub async fn update_customer(
    id: String,
    updated_customer: Customer,
    db: Db,
) -> Result<impl warp::Reply, Infallible> {
    let mut customers = db.lock().await;

    for customer in customers.iter_mut() {
        if customer.id == id {
            *customer = updated_customer;
            return Ok(StatusCode::OK);
        }
    }

    Ok(StatusCode::NOT_FOUND)
}

pub async fn delete_customer(id: String , db: Db) -> Result<impl warp::Reply, Infallible> {
    let mut customers = db.lock().await;

    let customer_count = customers.len();

    customers.retain(|customer| customer.id != id);

    let deleted = customers.len() != customer_count;
    if deleted {
        Ok(StatusCode::NO_CONTENT)
    }
    else {
        Ok(StatusCode::NOT_FOUND)
    }
}
