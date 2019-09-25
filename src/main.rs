#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

mod routes;
use rocket::config::{Config, Environment};
use rocket::Rocket;

fn get_config() -> Config {
    if cfg!(debug_assertions) {
        Config::build(Environment::Development).finalize().unwrap()
    } else {
        Config::build(Environment::Production)
            .port(8082)
            .finalize()
            .unwrap()
    }
}

fn rocket() -> Rocket {
    rocket::custom(get_config()).mount("/", routes![routes::index, routes::demangle])
}

fn main() {
    rocket().launch();
}
