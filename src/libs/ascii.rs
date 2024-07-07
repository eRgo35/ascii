use crate::libs::args;
use colored::Colorize;
use image;
use image::GenericImageView;

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

fn select_dominant_color(pixel: (u8, u8, u8), char_pixel: String) -> String {
    let (red, green, blue) = pixel;

    char_pixel.truecolor(red, green, blue).to_string()
}

pub fn to_ascii(img: &image::DynamicImage, args: &args::Args) -> String {
    let mut ascii_img = String::new();
    let (width, height) = img.dimensions();

    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y);
            let (mut red, mut green, mut blue) = (pixel[0], pixel[1], pixel[2]);

            let pixel_iterator: Vec<u16> = vec![red.into(), green.into(), blue.into()];
            let darkest_color: &u16 = pixel_iterator.iter().min().unwrap();
            let brightest_color: &u16 = pixel_iterator.iter().max().unwrap();

            let mut brightness = ((darkest_color + brightest_color) / 2) as u8;

            if args.invert {
                red = 255 - red;
                green = 255 - green;
                blue = 255 - blue;

                brightness = 255 - brightness;
            }

            let mut char_pixel = select_char(&brightness).repeat(args.pixel);

            if args.colorful {
                char_pixel = select_dominant_color((red, green, blue), char_pixel);
            }

            ascii_img.push_str(&char_pixel);
        }
        ascii_img.push('\n');
    }

    ascii_img
}
