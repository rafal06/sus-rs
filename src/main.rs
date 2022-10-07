#[macro_use] extern crate rocket;
use rocket::fs::{FileServer, NamedFile, relative};
use rocket::response::Redirect;
use serde::{Serialize, Deserialize};
use rocket::serde::json::Json;
use std::path::Path;

mod shortener;
use crate::shortener::gen_id;

mod database;
use crate::database::{save_to_db, get_url_by_id};

#[derive(Debug, Serialize, Deserialize)]
struct Url {
    url: String,
}

#[get("/")]
async fn index() -> Option<NamedFile> {
    let path = Path::new(relative!("frontend/dist/index.html"));
    NamedFile::open(path).await.ok()
}

#[post("/", data="<l_url>")]
async fn form_handler(l_url: Json<Url>) -> Json<Url> {
    println!("{:?}", l_url.url);
    let id = gen_id();
    save_to_db(&id, &l_url.url).await.unwrap();

    Json( Url {
        url: id,
    })
}

#[get("/<id>")]
async fn short_url(id: String) -> Option<Redirect> {
    match get_url_by_id(&id).await {
        Ok(url) => Some(Redirect::permanent(url)),
        Err(_) => None,
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/", routes![form_handler])
        .mount("/", routes![short_url])
        .mount("/", FileServer::from("frontend/dist"))
}
