#[macro_use] extern crate rocket;

mod shortener;
use crate::shortener::gen_id;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
}
