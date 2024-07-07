use crate::libs;

pub fn generator(
    image_path: String,
    invert: bool,
    colorful: bool,
    width: usize,
    height: usize,
    pixel: usize,
    noresize: bool,
    matrix: bool,
    nofill: bool,
    output: String,
    lightmap: String,
) {
    let path = image_path.clone();

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

    if !noresize && !nofill {
        img = libs::image::resize_image(&img, width, height);
        eprintln!("Image resized");
        libs::image::print_size(&img);
    }

    if !noresize && nofill {
        img = libs::image::resize_image_no_fill(&img, width, height);
        eprintln!("Image resized exact");
        libs::image::print_size(&img);
    }

    let ascii_img = libs::ascii::to_ascii(&img, &lightmap, pixel, invert, colorful, matrix);
    eprintln!("ASCII image created");

    if !output.is_empty() {
        libs::image::save_image(&ascii_img, &output);
        return;
    }

    print!("{ascii_img}");
}
