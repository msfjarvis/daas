extern crate cpp_demangle;
extern crate rocket;
use cpp_demangle::Symbol;
use rocket::http::RawStr;

use std::string::ToString;

#[get("/<symbol>")]
pub fn demangle(symbol: &RawStr) -> String {
    let sym = match Symbol::new(&symbol[..]) {
        Ok(result) => format!("{}", result),
        Err(_) => format!("Failed to demangle {}", symbol),
    };
    format!("{}\n", sym.to_string())
}

#[get("/")]
pub fn index() -> &'static str {
    "Make a GET call with /<mangled_symbol> to return the demangled form\n"
}
