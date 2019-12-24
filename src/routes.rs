use cpp_demangle::Symbol;
use rocket::http::RawStr;

#[get("/<symbol>")]
pub fn demangle(symbol: &RawStr) -> String {
    let sym = match Symbol::new(&symbol[..]) {
        Ok(result) => format!("{}", result),
        Err(_) => format!("Failed to demangle {}", symbol),
    };
    format!("{}\n", sym)
}

#[get("/<symbol>/json")]
pub fn demangle_as_json(symbol: &RawStr) -> String {
    let sym = match Symbol::new(&symbol[..]) {
        Ok(result) => format!("{}", result),
        Err(_) => format!("Failed to demangle {}", symbol),
    };
    format!("{{\"symbol\": \"{}\", \"result\": \"{}\"}}\n", symbol, sym)
}

#[get("/")]
pub fn index() -> &'static str {
    "Make a GET call with /<mangled_symbol> to return the demangled form\n"
}
