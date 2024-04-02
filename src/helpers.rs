use image::imageops::FilterType::{Lanczos3, Nearest};
// use image::imageops::{contrast, dither, BiLevel};
use image::io::Reader as ImageReader;
// use rusty_tesseract::{Args, Image};
// use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use uuid::Uuid;

const UPLOAD_DIR: &str = "./tmp";

// fn default_ocr_args() -> Args {
//     Args {
//         lang: "pol+eng".to_owned(),
//         config_variables: HashMap::from([(
//             "tessedit_char_whitelist".into(),
//             "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZęóąśłżźćń?,.! ".into(),
//         )]),
//         dpi: Some(150),
//         psm: Some(6),
//         oem: Some(3),
//     }
// }

pub fn write_file_sys(multipart_file_data: &Vec<u8>, ext: &str) -> String {
    let unique_id = Uuid::new_v4().to_string();
    let resolved_ext = ext.split("/").last().unwrap_or("");

    let file_path = UPLOAD_DIR.to_owned() + "/" + &unique_id + "." + resolved_ext;

    println!("Writing file to {}", file_path);

    let mut file = File::create(&file_path).unwrap();
    let _ = file.write_all(multipart_file_data);

    file_path
}

pub fn ocr_read_file_text(file_name: &str) -> String {
    let dynamic_image = ImageReader::open(file_name).unwrap().decode().unwrap();
    let mut dynamic_image = dynamic_image.grayscale();
    let mut dynamic_image = dynamic_image.adjust_contrast(100.);
    // let mut dynamic_image = dynamic_image.brighten();
    let mut dynamic_image = dynamic_image.resize(1200, 1200, Lanczos3);

    // let mut dynamic_image = dynamic_image
    //     .as_mut_luma8()
    //     .unwrap_or(dynamic_image.as_mut_luma_alpha8().unwrap());
    // dither(&mut dynamic_image, &BiLevel);
    dynamic_image.save("./tmp/cat.png").unwrap();

    // let mut lt = leptess::LepTess::new(None, "pol+eng").unwrap();
    // let _ = lt.set_image("./tmp/cat.png");
    // let output = lt.get_utf8_text().unwrap();

    let dynamic_image = ImageReader::open("./tmp/cat.png")
        .unwrap()
        .decode()
        .unwrap();

    // let img = Image::from_dynamic_image(&dynamic_image).unwrap();

    // let output = rusty_tesseract::image_to_string(&img, &default_ocr_args()).unwrap();
    println!("The String output for {} is: {:?}", file_name, "foo");

    String::from("foo")
}
