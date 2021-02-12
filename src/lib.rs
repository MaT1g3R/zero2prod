use rocket::Config;

#[macro_use]
extern crate rocket;

#[get("/health_check")]
fn health_check() {}

pub fn app(port: u16) -> rocket::Rocket {
    let mut config = Config::default();
    config.port = port;
    rocket::custom(config).mount("/", routes![health_check])
}
