use rocket::http::Status;
use rocket_contrib::json::Json;
use serde_json::Value;

use crate::models::db_client_wrapper::DBConnection;
use crate::models::restaurant::{Rating, Restaurant};
use crate::models::util;

#[get("/")]
pub fn all_restaurants(mut connection: DBConnection) -> Json<Value> {
    let (status, restaurants) = match connection
        .client
        .query("select * from restaurants order by name", &[])
    {
        Ok(rows) => (Status::Ok, Some(util::rows_to_values::<Restaurant>(rows))),
        Err(_) => (Status::NotFound, None),
    };

    util::build_response(status, restaurants)
}

#[get("/<name>")]
pub fn search(name: String, mut connection: DBConnection) -> Json<Value> {
    let (status, restaurants) = match connection.client.query(
        "select * from restaurants where name ilike $1 || '%' order by name",
        &[&name],
    ) {
        Ok(rows) => (Status::Ok, Some(util::rows_to_values::<Restaurant>(rows))),
        Err(_) => (Status::NotFound, None),
    };

    util::build_response(status, restaurants)
}

#[post("/new", format = "application/json", data = "<rating>")]
pub fn add_rating(rating: Json<Rating>, mut connection: DBConnection) -> Json<Value> {
    let (status, no_restaurant) = match connection.client.execute(
        "update restaurants set ratings = array_append(ratings, $2) where name = $1",
        &[&rating.name, &rating.rating],
    ) {
        Ok(rows) => (Status::Ok, rows == 0),
        Err(_) => (Status::NotFound, false),
    };

    let status = if no_restaurant {
        match connection.client.execute(
            "insert into restaurants values ($1, $2, default)",
            &[&rating.name, &vec![&rating.rating]],
        ) {
            Ok(_) => Status::Ok,
            Err(_) => Status::NotFound,
        }
    } else {
        status
    };

    util::build_simple_response(status)
}
