use rocket::{launch, routes};
use backend::routes::*;

mod algorithms;
mod bigint;
mod rsa;
mod backend;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![key_gen, encrypt, decrypt, sign, verify_sign])
        .configure(rocket::Config {
            port: 8080,
            address: "127.0.0.1".parse().unwrap(),
            ..rocket::Config::default()
        })
}
