#[macro_use]
extern crate actix_web;
extern crate diesel;

use std::{env, io};
use dotenv::dotenv;

use actix_web::{cookie::Key, middleware, App, HttpServer, web::Data};
use actix_session::{SessionMiddleware, storage::CookieSessionStore};
use diesel::r2d2::ConnectionManager;
use diesel::sqlite::SqliteConnection;
use r2d2::{Pool, PooledConnection};

mod constants;
mod response;
mod schema;
mod controller {
    pub mod tweet_controller;
    pub mod user_controller;
    pub mod login_controller;
}

mod orm {
    pub mod tweet_orm;
    pub mod user_orm;
}

pub type DBPool = Pool<ConnectionManager<SqliteConnection>>;
pub type DBPooledConnection = PooledConnection<ConnectionManager<SqliteConnection>>;

// The secret key would usually be read from a configuration file/environment variables.
fn get_secret_key() -> Key {
    return Key::generate();
}

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

    let secret_key = get_secret_key();

    HttpServer::new(move || {
        App::new()
            // Set up DB pool to be used with web::Data<Pool> extractor
            .app_data(Data::new(pool.clone()))
            // enable logger - always register actix-web Logger middleware last
            .wrap(middleware::Logger::default())
            .wrap(SessionMiddleware::new(CookieSessionStore::default(), secret_key.clone()))
            // register HTTP requests handlers
            .service(controller::tweet_controller::list)
            .service(controller::tweet_controller::create)
            .service(controller::user_controller::list)
            .service(controller::user_controller::get)
            .service(controller::user_controller::create)
            .service(controller::login_controller::login)
    })
    .bind("0.0.0.0:9090")?
    .run()
    .await
}