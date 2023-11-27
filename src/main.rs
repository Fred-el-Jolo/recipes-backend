#[macro_use]
extern crate actix_web;
extern crate diesel;

use std::{env, io};
use dotenv::dotenv;

use actix_web::{middleware, App, HttpServer, web::Data};
use diesel::r2d2::ConnectionManager;
use diesel::sqlite::SqliteConnection;
use r2d2::{Pool, PooledConnection};

mod constants;
mod response;
mod schema;
mod tweet;

pub type DBPool = Pool<ConnectionManager<SqliteConnection>>;
pub type DBPooledConnection = PooledConnection<ConnectionManager<SqliteConnection>>;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    // set up database connection pool
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool");

    HttpServer::new(move || {
        App::new()
            // Set up DB pool to be used with web::Data<Pool> extractor
            .app_data(Data::new(pool.clone()))
            // enable logger - always register actix-web Logger middleware last
            .wrap(middleware::Logger::default())
            // register HTTP requests handlers
            .service(tweet::list)
            // .service(tweet::get)
            .service(tweet::create)
            // .service(tweet::delete)
    })
    .bind("0.0.0.0:9090")?
    .run()
    .await
}