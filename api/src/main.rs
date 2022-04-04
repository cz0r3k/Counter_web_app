#[macro_use] extern crate rocket;
extern crate redis;
use redis::{Commands, Connection};
use rocket::http::Status;
use rocket::http::Header;
use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::serde::json::{json, Value};
use rocket_okapi::settings::UrlObject;
use rocket_okapi::{openapi, openapi_get_routes, rapidoc::*, swagger_ui::*};

const COUNTER_MAX_VALUE: i32 = 1000;
const COUNTER_MIN_VALUE: i32 = 0;
const COUNTER_INITIALIZATION_VALUE: i32 = 0;

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

fn get_connection()->redis::RedisResult<Connection>{
    redis::Client::open("redis://database").unwrap().get_connection()
}

fn return_counter()->redis::RedisResult<i32>{
    get_connection()?.get("counter_value")
}

fn set(value:i32) -> redis::RedisResult<()>{
    let _:() = get_connection().unwrap().set("counter_value", value)?;
    Ok(())
}

fn increment() -> redis::RedisResult<()>{
    let _: () = get_connection().unwrap().incr("counter_value",1)?;
    Ok(())
}

fn decrement() -> redis::RedisResult<()>{
    let _: () = get_connection().unwrap().decr("counter_value",1)?;
    Ok(())
}

fn initalize() -> redis::RedisResult<()> {
    let _ : () = get_connection()?.set_nx("counter_value", COUNTER_INITIALIZATION_VALUE)?;
    Ok(())
}

#[openapi(skip)]
#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

/// # Get current counter value
#[openapi(tag = "Counter")]
#[get("/get_counter")]
fn get_counter()-> Value {
    json!({"value":return_counter().unwrap()})
}

/// # Set counter value
#[openapi(tag = "Counter")]
#[post("/set_counter/<value>")]
fn set_counter(value: i32) -> Status{
    if !(COUNTER_MIN_VALUE..=COUNTER_MAX_VALUE).contains(&value){
        return Status::NotAcceptable;
    }
    let operation =set(value);
    if operation.is_err(){
        return Status::new(500);
    }
    Status::Ok
}

/// # Increment counter value by 1
#[openapi(tag = "Counter")]
#[post("/increment")]
fn increment_counter() -> Status{
    let counter = return_counter();
    if counter.is_err(){
        return Status::new(500);
    }
    if counter == Ok(COUNTER_MAX_VALUE) {
        return Status::NotAcceptable;
    }
    let operation = increment();
    if operation.is_err(){
        return Status::new(500);
    }
    Status::Ok
}

/// # Decrement counter value by 1
#[openapi(tag = "Counter")]
#[post("/decrement")]
fn decrement_counter()-> Status{
    let counter = return_counter();
    if counter.is_err(){
        return Status::new(500);
    }
    if counter == Ok(COUNTER_MIN_VALUE) {
        return Status::NotAcceptable;
    }
    let operation = decrement();
    if operation.is_err(){
        return Status::new(500);
    }
    Status::Ok
}


fn swagger_config() -> SwaggerUIConfig {
    SwaggerUIConfig {
        url: "../openapi.json".to_string(),
        ..Default::default()
    }
}

fn rapidoc_config() -> RapiDocConfig {
    RapiDocConfig {
        general: GeneralConfig {
            spec_urls: vec![UrlObject::new("General", "../openapi.json")],
            ..Default::default()
        },
        hide_show: HideShowConfig {
            allow_spec_url_load: false,
            allow_spec_file_load: false,
            ..Default::default()
        },
        ..Default::default()
    }
}

#[launch]
fn rocket() -> _ {
    initalize().unwrap();
    rocket::build()
    .mount("/api/", routes![index])
    .mount("/api/", openapi_get_routes![get_counter, set_counter, increment_counter, decrement_counter])
    .mount("/api/swagger/", make_swagger_ui(&swagger_config()))
    .mount("/api/rapidoc/", make_rapidoc(&rapidoc_config()))
    .attach(CORS)
}