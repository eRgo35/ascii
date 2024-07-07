use crate::libs;
use image::DynamicImage;
use nokhwa::{
    pixel_format::RgbFormat,
    utils::{CameraIndex, RequestedFormat, RequestedFormatType},
    Camera,
};

pub fn video(
    invert: bool,
    colorful: bool,
    width: usize,
    height: usize,
    pixel: usize,
    noresize: bool,
    matrix: bool,
    nofill: bool,
    lightmap: String,
) {
    let index = CameraIndex::Index(0);

    let requested =
        RequestedFormat::new::<RgbFormat>(RequestedFormatType::AbsoluteHighestFrameRate);

    let mut camera = Camera::new(index, requested).unwrap();

    camera.open_stream().unwrap();

    loop {
        let frame = camera.frame().unwrap();

        let decoded = frame.decode_image::<RgbFormat>().unwrap();
        let mut img: DynamicImage = decoded.into();

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

        clearscreen::clear().expect("Failed to clear screen");
        print!("{ascii_img}");
    }
}
