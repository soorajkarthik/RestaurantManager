use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::Request;

use crate::models::util;

pub struct DBConnection {
    pub client: postgres::Client,
}

impl<'a, 'r> FromRequest<'a, 'r> for DBConnection {
    type Error = postgres::Error;

    fn from_request(_request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        match util::get_db_connection() {
            Ok(client) => Outcome::Success(DBConnection { client }),
            Err(error) => Outcome::Failure((Status::NotFound, error)),
        }
    }
}
