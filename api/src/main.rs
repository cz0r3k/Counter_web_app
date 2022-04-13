#[macro_use]
extern crate rocket;
mod modules;
use modules::*;
use rocket_okapi::openapi_get_routes;
use rocket_okapi::{rapidoc::*, swagger_ui::*};

#[launch]
fn rocket() -> _ {
    if route::db::initalize().is_err() {
        eprintln!("Can not connect to databse");
    }
    rocket::build()
        .mount("/api/", routes![route::index])
        .mount(
            "/api/",
            openapi_get_routes![
                route::get_counter,
                route::set_counter,
                route::increment_counter,
                route::decrement_counter
            ],
        )
        .mount("/api/swagger/", make_swagger_ui(&swagger::swagger_config()))
        .mount("/api/rapidoc/", make_rapidoc(&swagger::rapidoc_config()))
        .attach(cors::Cors)
}
