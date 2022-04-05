extern crate redis;
use redis::{Commands, Connection};

const COUNTER_INITIALIZATION_VALUE: i32 = 0;

fn get_connection()->redis::RedisResult<Connection>{
    redis::Client::open("redis://database").unwrap().get_connection()
}

pub fn return_counter()->redis::RedisResult<i32>{
    get_connection()?.get("counter_value")
}

pub fn set(value:i32) -> redis::RedisResult<()>{
    let _:() = get_connection().unwrap().set("counter_value", value)?;
    Ok(())
}

pub fn increment() -> redis::RedisResult<()>{
    let _: () = get_connection().unwrap().incr("counter_value",1)?;
    Ok(())
}

pub fn decrement() -> redis::RedisResult<()>{
    let _: () = get_connection().unwrap().decr("counter_value",1)?;
    Ok(())
}

pub fn initalize() -> redis::RedisResult<()> {
    let _ : () = get_connection()?.set_nx("counter_value", COUNTER_INITIALIZATION_VALUE)?;
    Ok(())
}