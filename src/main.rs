use clap::Parser;
mod libs;

fn main() {
    println!("ASCII Generator\n----------------");

    let args = libs::args::Args::parse();
    let path = args.image.clone();

    let mut img = if !path.is_empty() {
        libs::image::load_image(&path)
    } else {
        match libs::image::load_image_from_stdin() {
            Ok(img) => img,
            Err(e) => {
                eprintln!("Error: {e}");
                std::process::exit(1);
            }
        }
    };

    libs::image::print_size(&img);

    if !args.noresize {
        img = libs::image::resize_image(&img, args.width, args.height);
        println!("Image resized");
        libs::image::print_size(&img);
    }

    let ascii_img = libs::ascii::to_ascii(&img, &args);
    println!("ASCII image created");

    println!("{ascii_img}");
}
