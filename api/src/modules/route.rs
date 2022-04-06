use rocket::http::Status;
use rocket_okapi::openapi;
use rocket::serde::json::{json, Value};
use rocket::response::Redirect;
pub mod db;

const COUNTER_MAX_VALUE: i32 = 1000;
const COUNTER_MIN_VALUE: i32 = 0;

#[openapi(skip)]
#[get("/")]
pub fn main_page() -> &'static str {
    "Hello from api"
}

#[openapi(skip)]
#[get("/")]
pub fn index() -> Redirect {
    Redirect::temporary("/api")
}

/// # Get current counter value
#[openapi(tag = "Counter")]
#[get("/get_counter")]
pub fn get_counter()-> Value {
    json!({"value":db::return_counter().unwrap()})
}

/// # Set counter value
#[openapi(tag = "Counter")]
#[post("/set_counter/<value>")]
pub fn set_counter(value: i32) -> Status{
    if !(COUNTER_MIN_VALUE..=COUNTER_MAX_VALUE).contains(&value){
        return Status::NotAcceptable;
    }
    let operation = db::set(value);
    if operation.is_err(){
        return Status::new(500);
    }
    Status::Ok
}

/// # Increment counter value by 1
#[openapi(tag = "Counter")]
#[post("/increment")]
pub fn increment_counter() -> Status{
    let counter = db::return_counter();
    if counter.is_err(){
        return Status::new(500);
    }
    if counter == Ok(COUNTER_MAX_VALUE) {
        return Status::NotAcceptable;
    }
    let operation = db::increment();
    if operation.is_err(){
        return Status::new(500);
    }
    Status::Ok
}

/// # Decrement counter value by 1
#[openapi(tag = "Counter")]
#[post("/decrement")]
pub fn decrement_counter()-> Status{
    let counter = db::return_counter();
    if counter.is_err(){
        return Status::new(500);
    }
    if counter == Ok(COUNTER_MIN_VALUE) {
        return Status::NotAcceptable;
    }
    let operation = db::decrement();
    if operation.is_err(){
        return Status::new(500);
    }
    Status::Ok
}