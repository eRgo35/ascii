use image;
use image::GenericImageView;

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