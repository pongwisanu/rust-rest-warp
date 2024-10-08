use pretty_env_logger;

mod routes;
mod handlers;
mod db;
mod config;
mod models;
mod services;


#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let db = db::init_db();
    let routes = routes::routes(db);

    warp::serve(routes)
        .run(([0,0,0,0], 3000))
        .await;
}
