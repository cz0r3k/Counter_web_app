#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/get_counter")]
fn get_counter(){
    todo!();
}

#[post("/set_counter/<value>")]
fn set_counter(value: i32){
    todo!();
}

#[post("/increment")]
fn increment(){
    todo!();
}

#[post("/decrement")]
fn decrement(){
    todo!();
}


#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}