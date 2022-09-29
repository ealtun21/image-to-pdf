use std::{fs::File, io::BufWriter};

use image_to_pdf::ImageToPdf;
use printpdf::image::{DynamicImage, self};

fn main() {
    let out_file = File::create("out.pdf").unwrap();

    ImageToPdf::new()
        .add_image(get_img("https://scans-manhwa.lowee.us/manga/Eleceed/0213-001.png").unwrap())
        .add_image(get_img("https://scans-manhwa.lowee.us/manga/Eleceed/0186-002.png").unwrap())
        .add_image(get_img("https://wallpapercave.com/wp/bfwAi41.jpg").unwrap())
        .create_pdf(&mut BufWriter::new(out_file))
        .unwrap();
}

pub fn get_img(url: &str) -> Result<DynamicImage, reqwest::Error> {
    Ok(
        image::load_from_memory(&reqwest::blocking::get(url)?.bytes()?)
            .expect("Failed to load image"),
    )
}