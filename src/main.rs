#[macro_use]
extern crate rocket;

#[launch]
fn rocket() -> rocket::Rocket {
    zero2prod::app(8000)
}
