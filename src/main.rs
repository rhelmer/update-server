#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;

extern crate iron;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use iron::prelude::*;
use iron::status;
use iron::mime::Mime;


// FIXME share all these structs w/ server.
#[derive(Serialize)]
struct AppConfig {
    product: String,
    version: i32,
    platform: String,
    locale: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Update {
    update_type: String,
    url: String,
    hash_function: String,
    hash_value: String,
    size: i32,
    version: i32,
}

#[derive(Serialize)]
struct ResultMessage {
    update_type: String,
    download_path: String,
}

#[derive(Serialize)]
struct SuccessUpdateStatus {
    update_type: String,
    version: i32,
}

#[derive(Serialize)]
struct FailedUpdateStatus {
    update_type: String,
    url: String,
    hash_function: String,
    hash_value: String,
    size: i32,
    version: i32,
    download_path: String,
}

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

fn main() {
    let listen = "localhost:8000";

    println!("Update server listening on {}", listen);

    Iron::new(|_: &mut Request| {
        let content_type = "application/json".parse::<Mime>().unwrap();

        // FIXME mock update response for now.
        // This will come from the DB, soon.
        let update = Update {
            update_type: "blocklist".to_string(),
            url: "http://localhost:8080/src/blah.zip".to_string(),
            hash_function: "sha512".to_string(),
            hash_value: "abc123".to_string(),
            size: 1024,
            version: 1001,
        };
        let json_response = serde_json::to_string(&update).unwrap();

        Ok(Response::with((content_type, status::Ok, json_response)))
    }).http(listen).unwrap();
}