use image::io::Reader;
use image;
use image::GenericImageView;
use std::io::{self, BufReader, BufRead, Cursor};
use atty::Stream;

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

    println!("Image loaded: stdin");

    reader.decode()
}

pub fn load_image(file_name: &str) -> image::DynamicImage {
  let img = image::open(file_name).expect("File not found!");
  println!("Image loaded: {file_name}");
  
  img
}

pub fn print_size(img: &image::DynamicImage) {
  let (width, height) = img.dimensions();
  println!("Image dimensions: {width}x{height}");
}

pub fn resize_image(img: &image::DynamicImage, nwidth: usize, nheight: usize) -> image::DynamicImage {
  img.resize(nwidth as u32, nheight as u32, image::imageops::FilterType::Lanczos3)
}