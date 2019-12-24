#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;
extern crate cpp_demangle;

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
    rocket::custom(get_config()).mount(
        "/",
        routes![routes::index, routes::demangle, routes::demangle_as_json],
    )
}

fn main() {
    rocket().launch();
}

#[cfg(test)]
mod test {
    use super::rocket;
    use rocket::http::Status;
    use rocket::local::Client;

    #[test]
    fn test_hello() {
        let client = Client::new(rocket()).unwrap();
        let mut response = client.get("/").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(
            response.body_string(),
            Some("Make a GET call with /<mangled_symbol> to return the demangled form\n".into())
        );
    }

    #[test]
    fn test_successful_demangle() {
        let client = Client::new(rocket()).unwrap();
        let mut response = client.get("/_ZN6icu_6011StringPieceC1EPKc").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(
            response.body_string(),
            Some("icu_60::StringPiece::StringPiece(char const*)\n".into())
        );
    }

    #[test]
    fn test_successful_demangle_as_json() {
        let client = Client::new(rocket()).unwrap();
        let mut response = client.get("/_ZN6icu_6011StringPieceC1EPKc/json").dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(
            response.body_string(),
            Some("{\"symbol\": \"_ZN6icu_6011StringPieceC1EPKc\", \"result\": \"icu_60::StringPiece::StringPiece(char const*)\"}\n".into())
        );
    }

    #[test]
    fn test_invalid_symbol() {
        let client = Client::new(rocket()).unwrap();
        let fake_symbol = "no_way_this_is_a_valid_symbol";
        let mut response = client
            .get(format!("/{}", "no_way_this_is_a_valid_symbol"))
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(
            response.body_string(),
            Some(format!("Failed to demangle {}\n", fake_symbol))
        );
    }
}
