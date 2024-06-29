use clap::Parser;
mod libs;

fn main() {
    println!("ASCII Generator\n----------------");
    
    let args = libs::args::Args::parse();

    let img = libs::image::load_image(&args.image);
    libs::image::print_size(&img);

    let resized_img = libs::image::resize_image(&img, args.width, args.height);
    println!("Image resized");
    libs::image::print_size(&resized_img);

    let ascii_img = libs::ascii::to_ascii(&resized_img, &args);
    println!("ASCII image created");
    
    println!("{ascii_img}");
}