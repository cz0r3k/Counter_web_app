#[macro_use] extern crate rocket;
use rocket::http::Status;
extern crate redis;
use redis::{Commands, Connection};
const COUNTER_INITIALIZATION_VALUE: i32 = 0;
const COUNTER_MAX_VALUE: i32 = 1000;
const COUNTER_MIN_VALUE: i32 = 0;


fn get_connection()->redis::RedisResult<Connection>{
    redis::Client::open("redis://database").unwrap().get_connection()
}

fn return_counter()->redis::RedisResult<i32>{
    get_connection()?.get("counter_value")
}

fn initalize_counter() -> redis::RedisResult<()> {
    let _ : () = get_connection()?.set_nx("counter_value", COUNTER_INITIALIZATION_VALUE)?;
    Ok(())
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/get_counter")]
fn get_counter()-> String {
   format!("value: {}", return_counter().unwrap())
}

#[post("/set_counter/<value>")]
fn set_counter(value: i32) -> Status{
    if !(COUNTER_MIN_VALUE..=COUNTER_MAX_VALUE).contains(&value){
        return Status::NotAcceptable;
    }
    let _:() = get_connection().unwrap().set_nx("counter_value", value).unwrap();
    Status::Ok
}

#[post("/increment")]
fn increment() -> Status{
    let counter = return_counter();
    if counter.is_err(){
        return Status::new(500);
    }
    if counter == Ok(COUNTER_MAX_VALUE) {
        return Status::NotAcceptable;
    }
    let _: () = get_connection().unwrap().incr("counter_value",1).unwrap();
    Status::Ok
}

#[post("/decrement")]
fn decrement()-> Status{
    if return_counter() == Ok(COUNTER_MIN_VALUE) {
        return Status::NotAcceptable;
    }
    let _: () = get_connection().unwrap().decr("counter_value",1).unwrap();
    Status::Ok
}

#[launch]
fn rocket() -> _ {
    initalize_counter().unwrap();
    rocket::build().mount("/", routes![index, get_counter, set_counter, increment, decrement])
}