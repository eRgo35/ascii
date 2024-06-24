use std::env;
use image;
use image::GenericImageView;

fn parse_args() -> Vec<String> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: {} <image_file>", args[0]);
        std::process::exit(1);
    }

    args
}

fn load_image(file_name: &str) -> image::DynamicImage {
    let img = image::open(file_name).expect("File not found!");
    println!("Image loaded: {file_name}");
    
    img
}

fn print_size(img: &image::DynamicImage) {
    let (width, height) = img.dimensions();
    println!("Image dimensions: {width}x{height}");
}

fn select_char(brightness: &u8) -> &str {
    match brightness {
        0..=25 => "  ",
        26..=50 => "..",
        51..=75 => "::",
        76..=100 => "--",
        101..=125 => "==",
        126..=150 => "++",
        151..=175 => "**",
        176..=200 => "##",
        201..=225 => "%%",
        226..=255 => "@@",
    }
}

fn to_ascii(img: &image::DynamicImage) -> String {
    let (width, height) = img.dimensions();
    
    let mut ascii_img = String::new();
    
    for y in 0..height {
        for x in 0..width {
            let pixel = img.get_pixel(x, y);

            let red = pixel[0] as u16;
            let green = pixel[1] as u16;
            let blue = pixel[2] as u16;
            
            let brightness = ((red + green + blue) / 3) as u8;

            ascii_img.push_str(select_char(&brightness));
        }
        ascii_img.push('\n');
    }
    
    ascii_img
}

fn resize_image(img: &image::DynamicImage, nwidth: u32, nheight: u32) -> image::DynamicImage {
    img.resize(nwidth, nheight, image::imageops::FilterType::Lanczos3)
}

fn main() {
    println!("ASCII Generator\n----------------");
    
    let args: Vec<String> = parse_args();

    let img = load_image(&args[1]);
    print_size(&img);

    let resized_img = resize_image(&img, 80, 25);
    println!("Image resized");
    print_size(&resized_img);

    let ascii_img = to_ascii(&resized_img);
    println!("ASCII image created");
    
    println!("{ascii_img}");
}