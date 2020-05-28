#![feature(proc_macro_hygiene, decl_macro)]
extern crate postgres;
extern crate postgres_types;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
extern crate rocket_cors;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use models::util;
use postgres::Error;
use rocket_cors::{CorsOptions};
use std::fs;

mod models;
mod requests;

fn main() -> Result<(), Error> {
    dotenv::dotenv().ok();

    match util::get_db_connection() {
        Ok(mut client) => {
            client.batch_execute(&fs::read_to_string("src/sql/init_tables.sql").unwrap())?
        }
        Err(err) => return Err(err),
    }

    let cors = CorsOptions::default().to_cors().unwrap();

    rocket::ignite()
        .mount("/test", routes![util::test])
        .mount(
            "/",
            routes![
                requests::restaurant::all_restaurants,
                requests::restaurant::search,
                requests::restaurant::add_rating,
            ],
        )
        .mount("/", rocket_cors::catch_all_options_routes())
        .manage(cors.clone())
        .attach(cors)
        .launch();

    Ok(())
}
