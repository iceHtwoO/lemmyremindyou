use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    match PgConnection::establish(&database_url) {
        Ok(x) => x,
        Err(e) => panic!("Error connecting to {}! {}", database_url, e.to_string())
    }
}

pub mod models;
pub mod schema;