use axum::extract::Multipart;
use axum::response::Html;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

pub async fn new() -> Html<&'static str> {
    let form = r#"
      <form method="post" action="/ocr" enctype="multipart/form-data">
          <input type="file" name="file" />
          <button type="submit">Submit</button>
      </form>
    "#;

    Html(&form)
}

pub async fn create(mut multipart: Multipart) {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let filename = field.file_name().unwrap().to_string();
        let data = field.bytes().await.unwrap();

        let build_file_path = format!("{}/{}", "./tmp", &filename);

        let mut upload_file = File::create(build_file_path).await.unwrap();

        upload_file.write_all(&data).await.unwrap();

        println!("Length of `{}` is {} bytes", name, data.len());
    }
}
