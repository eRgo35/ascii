use colored::Colorize;
use image;
use image::GenericImageView;

fn calculate_brightness((red, green, blue): (u8, u8, u8), mapping: &str) -> u8 {
    let pixel_iterator: Vec<u16> = vec![red.into(), green.into(), blue.into()];
    let darkest_color: &u16 = pixel_iterator.iter().min().unwrap();
    let brightest_color: &u16 = pixel_iterator.iter().max().unwrap();

    let mapping: u8 = match mapping {
        "fullbright" => 255,
        "luminosity" => (0.21 * red as f32 + 0.72 * green as f32 + 0.07 * blue as f32) as u8,
        "minmax" | "average" | _ => ((darkest_color + brightest_color) / 2) as u8,
    };

    mapping
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

fn select_dominant_color(pixel: (u8, u8, u8), char_pixel: String) -> String {
    let (red, green, blue) = pixel;

    char_pixel.truecolor(red, green, blue).to_string()
}

pub fn to_ascii(
    img: &image::DynamicImage,
    lightmap: &str,
    pixel_format: usize,
    invert: bool,
    colorful: bool,
    matrix: bool,
) -> String {
    let mut ascii_img = String::new();
    let (width, height) = img.dimensions();

    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y);
            let (mut red, mut green, mut blue) = (pixel[0], pixel[1], pixel[2]);

            let mut brightness = calculate_brightness((red, green, blue), lightmap);

            if invert {
                red = 255 - red;
                green = 255 - green;
                blue = 255 - blue;
                brightness = 255 - brightness;
            }

            let mut char_pixel = select_char(&brightness).repeat(pixel_format);

            if colorful {
                char_pixel = select_dominant_color((red, green, blue), char_pixel);
            }

            if matrix {
                char_pixel = char_pixel.bright_green().to_string();
            }

            ascii_img.push_str(&char_pixel);
        }
        ascii_img.push('\n');
    }

    ascii_img
}
