#[macro_use]
extern crate rocket;

// use rocket::form::FromForm;

use rocket::http::{ContentType, Status};
use rocket::response::status::Custom;
use rocket::Data;

use rocket_raw_response::RawResponse;

use rocket_multipart_form_data::{
    mime, multer, MultipartFormData, MultipartFormDataError, MultipartFormDataField,
    MultipartFormDataOptions,
};

// use rocket_raw_response::RawResponse;

pub mod compressor;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/", data = "<data>")]
async fn compress(
    content_type: &ContentType,
    data: Data<'_>,
) -> Result<RawResponse, Custom<String>> {
    let options = MultipartFormDataOptions {
        max_data_bytes: 33 * 1024 * 1024,
        allowed_fields: vec![MultipartFormDataField::raw("image")
            .size_limit(32 * 1024 * 1024)
            .content_type_by_string(Some(mime::IMAGE_STAR))
            .unwrap()],
        ..MultipartFormDataOptions::default()
    };

    let mut multipart_form_data = match MultipartFormData::parse(content_type, data, options).await
    {
        Ok(multipart_form_data) => multipart_form_data,
        Err(err) => {
            match err {
                MultipartFormDataError::DataTooLargeError(e) => {
                    return Err(Custom(Status::BadRequest, e.to_string()));
                }
                MultipartFormDataError::DataTypeError(_) => {
                    return Err(Custom(
                        Status::BadRequest,
                        "This is not an image".to_string(),
                    ));
                }
                MultipartFormDataError::MulterError(multer::Error::IncompleteFieldData {
                    ..
                })
                | MultipartFormDataError::MulterError(multer::Error::IncompleteHeaders {
                    ..
                }) => {
                    // may happen when we set the max_data_bytes limitation
                    return Err(Custom(
                        Status::BadRequest,
                        "The request body seems too large.".to_string(),
                    ));
                }
                _ => panic!("{:?}", err),
            }
        }
    };

    let image = multipart_form_data.raw.remove("image");

    match image {
        Some(mut image) => {
            let compressed = compressor::Compressor::new();
            let raw = image.remove(0);
            let content_type = raw.content_type;
            let file_name = raw.file_name.unwrap_or_else(|| "Image".to_string());
            let data = raw.raw;
            let result = compressed.compress_from_memory(&data);

            match result {
                Ok(data) => Ok(RawResponse::from_vec(data, Some(file_name), content_type)),
                Err(e) => Err(Custom(Status::BadRequest, e.to_string())),
            }
        }
        None => Err(Custom(Status::BadRequest, "No image provided".to_string())),
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .mount("/compress", routes![compress])
}
