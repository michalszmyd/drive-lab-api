use actix_multipart_extract::{File, Multipart, MultipartForm};
use actix_web::{error, get, post, web, HttpResponse, Responder, Result};
use derive_more::{Display, Error};
use serde::{Deserialize, Serialize};

use crate::helpers::{ocr_read_file_text, write_file_sys};

#[derive(Deserialize, MultipartForm, Debug)]
pub struct OcrFormData {
    #[multipart(max_size = 5MB)]
    file: File,
}

#[derive(Serialize)]
struct OcrResponse {
    body: String,
}

#[derive(Debug, Display, Error)]
#[display(fmt = "Unprocessable entity: {}", message)]
struct UnprocessableEntityError {
    message: String,
}
impl error::ResponseError for UnprocessableEntityError {}

#[derive(Serialize)]
struct Success {
    message: String,
}

#[get("/ocr")]
pub async fn index() -> Result<impl Responder> {
    let form = r#"
        <!doctype html>
        <html>
            <head></head>
            <body>
                <form action="/ocr" method="post" enctype="multipart/form-data">
                    <label>
                        Upload file:
                        <input type="file" name="file">
                    </label>

                    <input type="submit" value="Upload files">
                </form>
            </body>
        </html>
    "#;

    Ok(HttpResponse::Ok().body(form))
}

#[post("/ocr")]
pub async fn create(
    payload: Multipart<OcrFormData>,
) -> Result<impl Responder, UnprocessableEntityError> {
    println!("Started OCR POST");

    let uploaded_file_path = write_file_sys(&payload.file.bytes, &payload.file.content_type);

    let ocr_value = ocr_read_file_text(&uploaded_file_path);

    let result = OcrResponse { body: ocr_value };

    Ok(web::Json(result))
}

// pub async fn save_file(mut payload: Multipart, file_path: String) -> Option<bool> {
//     // iterate over multipart stream
//     while let Ok(Some(mut field)) = payload.try_next().await {
//         let content_type = field.content_disposition().unwrap();
//         //let filename = content_type.get_filename().unwrap();
//         let filepath = format!(".{}", file_path);

//         // File::create is blocking operation, use threadpool
//         let mut f = web::block(|| std::fs::File::create(filepath))
//             .await
//             .unwrap();

//         // Field in turn is stream of *Bytes* object
//         while let Some(chunk) = field.next().await {
//             let data = chunk.unwrap();
//             // filesystem operations are blocking, we have to use threadpool
//             f = web::block(move || f.write_all(&data).map(|_| f))
//                 .await
//                 .unwrap();
//         }
//     }

//     Some(true)
// }
