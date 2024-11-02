use std::env;

use backend::routes::*;
use rocket::{launch, routes};

mod algorithms;
mod backend;
mod bigint;
mod rsa;

#[launch]
fn rocket() -> _ {
    let port: u16 = match env::var("RUST_API_PORT") {
        Ok(value) => {
            if let Ok(value) = u16::from_str_radix(&value, 10) {
                value
            } else {
                8080
            }
        },
        Err(_) => 8080,
    };
    rocket::build()
        .mount("/", routes![key_gen, encrypt, decrypt, sign, verify_sign])
        .configure(rocket::Config {
            port,
            address: "127.0.0.1".parse().unwrap(),
            ..rocket::Config::default()
        })
}
