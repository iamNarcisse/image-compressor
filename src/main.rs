#[macro_use]
extern crate rocket;

use rocket::http::ContentType;
use rocket::response::status::Custom;
use rocket::Data;
use rocket_raw_response::RawResponse;

mod compressor;
mod service;

use service::CompressImageService;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/", data = "<data>")]
async fn compress(
    content_type: &ContentType,
    data: Data<'_>,
) -> Result<RawResponse, Custom<String>> {
    let srv = CompressImageService::new();
    let result = srv.compress(content_type, data).await;
    match result {
        Ok(result) => Ok(result),
        Err(err) => Err(err),
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/compress", routes![compress])
}
