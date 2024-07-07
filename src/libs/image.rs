use image::io::Reader;
use image;
use image::GenericImageView;
use std::io::{self, BufReader, BufRead, Cursor};
use atty::Stream;
use std::fs;

pub fn load_image_from_stdin() -> Result<image::DynamicImage, image::ImageError> {
    let mut buffer: Vec<u8> = Vec::new();
    let mut raw_reader: Box<dyn BufRead> = if atty::is(Stream::Stdin) {
      eprintln!("Error: No image provided");
      std::process::exit(1);
    } else {  
      Box::new(BufReader::new(io::stdin()))
    };

    raw_reader.read_to_end(&mut buffer).unwrap();
    
    let reader = Reader::new(Cursor::new(buffer))
    .with_guessed_format()
    .expect("Failed to read image format");

    eprintln!("Image loaded: stdin");

    reader.decode()
}

pub fn load_image(file_name: &str) -> image::DynamicImage {
  let img = image::open(file_name).expect("File not found!");
  eprintln!("Image loaded: {file_name}");
  
  img
}

pub fn save_image(img: &String, file_name: &str) {
  fs::write(file_name, img).expect("Unable to write file");
  eprintln!("Image saved: {file_name}");
}

pub fn print_size(img: &image::DynamicImage) {
  let (width, height) = img.dimensions();
  eprintln!("Image dimensions: {width}x{height}");
}

pub fn resize_image(img: &image::DynamicImage, nwidth: usize, nheight: usize) -> image::DynamicImage {
  img.resize(nwidth as u32, nheight as u32, image::imageops::FilterType::Lanczos3)
}

pub fn resize_image_no_fill(img: &image::DynamicImage, nwidth: usize, nheight: usize) -> image::DynamicImage {
  img.resize_exact(nwidth as u32, nheight as u32, image::imageops::FilterType::Lanczos3)
}