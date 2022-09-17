#[macro_use] extern crate rocket;
use serde::{Serialize, Deserialize};
use rocket::serde::json::Json;

mod shortener;
use crate::shortener::gen_id;

#[derive(Debug, Serialize, Deserialize)]
struct Url {
    url: String,
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/", data="<l_url>")]
fn form_handler(l_url: Json<Url>) -> Json<Url> {
    println!("{:?}", l_url.url);

    Json( Url {
        url: gen_id(),
    })
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![form_handler])
}
