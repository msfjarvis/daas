#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
extern crate cpp_demangle;

use cpp_demangle::Symbol;
use rocket::config::{Config, Environment};
use rocket::http::RawStr;
use std::string::ToString;

#[get("/<symbol>")]
fn demangle(symbol: &RawStr) -> String {
    let sym = match Symbol::new(&symbol[..]) {
        Ok(result) => format!("{}", result),
        Err(_) => format!("Failed to demangle {}", symbol),
    };
    format!("{}\n", sym.to_string())
}

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

#[get("/")]
fn index() -> &'static str {
    "Make a GET call with /<mangled_symbol> to return the demangled form\n"
}

fn main() {
    rocket::custom(get_config())
        .mount("/", routes![index, demangle])
        .launch();
}
