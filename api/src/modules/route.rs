use rocket::http::Status;
use rocket_okapi::openapi;
use rocket::serde::json::{json, Value};
pub mod db;

const COUNTER_MAX_VALUE: i32 = 1000;
const COUNTER_MIN_VALUE: i32 = 0;

#[openapi(skip)]
#[get("/")]
pub fn index() -> &'static str {
    "Hello from api"
}

/// # Get current counter value
///
/// Return current counter value
#[openapi(tag = "Counter")]
#[get("/get_counter")]
pub fn get_counter()-> Result<Value,Status> {
    Ok(json!({"value":db::return_counter().unwrap()}))
}

/// # Set counter value
///
/// Return current counter value
#[openapi(tag = "Counter")]
#[post("/set_counter/<value>")]
pub fn set_counter(value: i32) -> Result<Value,Status>{
    if !(COUNTER_MIN_VALUE..=COUNTER_MAX_VALUE).contains(&value){
        return Err(Status::NotAcceptable);
    }
    let operation = db::set(value);
    if operation.is_err(){
        return Err(Status::new(500));
    }
    Ok(json!({"value":db::return_counter().unwrap()}))
}

/// # Increment counter value by 1
///
/// Return current counter value
#[openapi(tag = "Counter")]
#[post("/increment")]
pub fn increment_counter() -> Result<Value,Status>{
    let counter = db::return_counter();
    if counter.is_err(){
        return Err(Status::new(500));
    }
    if counter == Ok(COUNTER_MAX_VALUE) {
        return Err(Status::NotAcceptable);
    }
    let operation = db::increment();
    if operation.is_err(){
        return Err(Status::new(500));
    }
    Ok(json!({"value":db::return_counter().unwrap()}))
}

/// # Decrement counter value by 1
///
/// Return current counter value
#[openapi(tag = "Counter")]
#[post("/decrement")]
pub fn decrement_counter()-> Result<Value,Status>{
    let counter = db::return_counter();
    if counter.is_err(){
        return Err(Status::new(500));
    }
    if counter == Ok(COUNTER_MIN_VALUE) {
        return Err(Status::NotAcceptable);
    }
    let operation = db::decrement();
    if operation.is_err(){
        return Err(Status::new(500));
    }
    Ok(json!({"value":db::return_counter().unwrap()}))
}