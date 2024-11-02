use std::env;

use backend::routes::*;
use rocket::{launch, routes};
use rocket_cors::{AllowedOrigins, Cors, CorsOptions};

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
    let cors = CorsOptions::default().allowed_origins(AllowedOrigins::all());
    rocket::build()
        .attach(cors.to_cors().unwrap())
        .mount("/", routes![key_gen, encrypt, decrypt, sign, verify_sign])
        .configure(rocket::Config {
            port,
            address: "0.0.0.0".parse().unwrap(),
            ..rocket::Config::default()
        })
}
