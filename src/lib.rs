#![allow(clippy::unit_arg)]
use rocket::{request::Form, Config};

#[macro_use]
extern crate rocket;

#[derive(FromForm)]
struct FormData {
    email: String,
    name: String,
}

#[get("/health_check")]
fn health_check() {}

#[post("/subscriptions", data = "<_form>")]
fn subscribe(_form: Form<FormData>) {}

pub fn app(port: u16) -> rocket::Rocket {
    let config = Config {
        port,
        ..Config::default()
    };
    rocket::custom(config).mount("/", routes![health_check, subscribe])
}
