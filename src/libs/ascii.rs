use image;
use image::GenericImageView;
use crate::libs::args;
use colored::Colorize;

fn grayscale_ascii(img: &image::DynamicImage, width: u32, height: u32, invert: bool, pixel_size: usize) -> String {
  let mut ascii_img = String::new();
  
  for y in 0..height {
    for x in 0..width {
      let pixel = img.get_pixel(x, y);

      let red = pixel[0];
      let green = pixel[1];
      let blue = pixel[2];

      let pixel_iterator: Vec<u16> = vec![red.into(), green.into(), blue.into()];
      let darkest_color: &u16 = pixel_iterator.iter().min().unwrap();
      let brightest_color: &u16 = pixel_iterator.iter().max().unwrap();

      let mut brightness = ((darkest_color  + brightest_color) / 2) as u8;

      if invert {
        brightness = 255 - brightness;
      }

      let char_pixel = select_char(&brightness).repeat(pixel_size);

      ascii_img.push_str(&char_pixel);
    }
    ascii_img.push('\n');
  }

  ascii_img
}

fn colorful_ascii(img: &image::DynamicImage, width: u32, height: u32, pixel_size: usize) -> String {
  let mut ascii_img = String::new();
  
  for y in 0..height {
    for x in 0..width {
      let pixel = img.get_pixel(x, y);

      let red = pixel[0];
      let green = pixel[1];
      let blue = pixel[2];

      let pixel_iterator: Vec<u16> = vec![red.into(), green.into(), blue.into()];
      let darkest_color: &u16 = pixel_iterator.iter().min().unwrap();
      let brightest_color: &u16 = pixel_iterator.iter().max().unwrap();

      let brightness = ((darkest_color  + brightest_color) / 2) as u8;

      let mut char_pixel = select_char(&brightness).repeat(pixel_size);

      char_pixel = select_dominant_color((red, green, blue), char_pixel);

      ascii_img.push_str(&char_pixel);
    }
    ascii_img.push('\n');
  }

  ascii_img
}

fn select_char(brightness: &u8) -> String {
  let char = match brightness {
      0..=25 => " ",
      26..=50 => ".",
      51..=75 => ":",
      76..=100 => "-",
      101..=125 => "=",
      126..=150 => "+",
      151..=175 => "*",
      176..=200 => "#",
      201..=225 => "%",
      226..=255 => "@",
  };
  
  char.into()
}

fn select_dominant_color(pixel : (u8, u8, u8), char_pixel: String) -> String {
  let (red, green, blue) = pixel;

  let char_pixel = char_pixel.truecolor(red, green, blue).to_string();

  char_pixel
}

pub fn to_ascii(img: &image::DynamicImage, args: &args::Args) -> String {
  let (width, height) = img.dimensions();

  if args.colorful {
    return colorful_ascii(img, width, height, args.pixel);
  }

  grayscale_ascii(img, width, height, args.invert, args.pixel)  
}