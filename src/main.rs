use rocket::form::{Form, FromForm};
use rocket::fs::TempFile;
pub mod compressor;

#[macro_use]
extern crate rocket;

#[derive(FromForm)]
struct Upload<'r> {
    save: bool,
    file: TempFile<'r>,
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/", data = "<upload>")]
fn compress(upload: Form<Upload<'_>>) -> Result<(), &'static str> {
    let engine = compressor::Compressor::new();
    let response = engine.compress("compress.png");
    match response {
        Ok(data) => Ok(data),
        Err(e) => Err("Failed to compress image "),
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/compress", routes![compress])
}
