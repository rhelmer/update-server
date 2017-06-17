extern crate iron;

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use iron::prelude::*;
use iron::status;
use iron::mime::Mime;


// FIXME share this with the client.
#[derive(Serialize)]
struct AppConfig {
    product: &'static str,
    version: i32,
    platform: &'static str,
    locale: &'static str,
}

// FIXME share this with the client.
#[derive(Serialize, Deserialize, Debug)]
struct Update {
    update_type: &'static str,
    url: &'static str,
    hash_function: &'static str,
    hash_value: &'static str,
    size: i32,
    version: i32,
}

// FIXME share this with the client.
#[derive(Serialize)]
struct SuccessUpdateStatus {
    update_type: &'static str,
    version: i32,
}

// FIXME share this with the client.
#[derive(Serialize)]
struct FailedUpdateStatus {
    update_type: &'static str,
    url: &'static str,
    hash_function: &'static str,
    hash_value: &'static str,
    size: i32,
    version: i32,
    download_path: String,
}

fn main() {
    let listen = "localhost:8000";

    println!("Update server listening on {}", listen);

    Iron::new(|_: &mut Request| {
        let content_type = "application/json".parse::<Mime>().unwrap();

        // FIXME mock update response for now.
        let update = Update {
            update_type: "blocklist",
            url: "http://localhost:8080/src/blah.zip",
            hash_function: "sha512",
            hash_value: "abc123",
            size: 1024,
            version: 1000,
        };
        let json_response = serde_json::to_string(&update).unwrap();

        Ok(Response::with((content_type, status::Ok, json_response)))
    }).http(listen).unwrap();
}