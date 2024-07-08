use std::time::Duration;

use crate::libs;
use image::{DynamicImage, ImageBuffer};

use ffmpeg_next::format::{input, Pixel};
use ffmpeg_next::media::Type;
use ffmpeg_next::software::scaling::{context::Context, flag::Flags};
use ffmpeg_next::util::frame::video::Video;

pub fn video(
    input_path: String,
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
    ffmpeg_next::init().unwrap();

    if input_path.is_empty() {
        eprintln!("Error: No input path provided");
        std::process::exit(1);
    }

    if let Ok(mut ictx) = input(&input_path) {
        let input = ictx
            .streams()
            .best(Type::Video)
            .ok_or(ffmpeg_next::Error::StreamNotFound)
            .expect("Failed to find video stream");
        let video_stream_index = input.index();

        let context_decoder =
            ffmpeg_next::codec::context::Context::from_parameters(input.parameters()).unwrap();
        let mut decoder = context_decoder.decoder().video().unwrap();

        let mut scaler = Context::get(
            decoder.format(),
            decoder.width(),
            decoder.height(),
            Pixel::RGB24,
            decoder.width(),
            decoder.height(),
            Flags::BILINEAR,
        )
        .unwrap();

        let frame_rate = input.avg_frame_rate();
        let frame_duration = Duration::from_secs_f64(frame_rate.denominator() as f64 / frame_rate.numerator() as f64);

        let mut frame_index = 0;

        let mut receive_and_process_decoded_frames =
            |decoder: &mut ffmpeg_next::decoder::Video| -> Result<(), ffmpeg_next::Error> {
                let mut decoded = Video::empty();
                while decoder.receive_frame(&mut decoded).is_ok() {
                    let start = std::time::Instant::now();

                    let mut rgb_frame = Video::empty();
                    scaler.run(&decoded, &mut rgb_frame)?;
                    
                    let vwidth = rgb_frame.width();
                    let vheight = rgb_frame.height();

                    let data = rgb_frame.data(0);

                    let buf = ImageBuffer::from_raw(vwidth, vheight, data.to_vec()).unwrap();

                    let mut img = DynamicImage::ImageRgb8(buf);

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

                    let ascii_img =
                        libs::ascii::to_ascii(&img, &lightmap, pixel, invert, colorful, matrix);
                    eprintln!("ASCII image created");

                    clearscreen::clear().expect("Failed to clear screen");
                    print!("{ascii_img}");

                    frame_index += 1;

                    let elapsed = start.elapsed();
                    if elapsed < frame_duration {
                        std::thread::sleep(frame_duration - elapsed);
                    }
                }

                Ok(())
            };

        for (stream, packet) in ictx.packets() {
            if stream.index() == video_stream_index {
                decoder.send_packet(&packet).unwrap();
                receive_and_process_decoded_frames(&mut decoder).unwrap();
            }
        }

        decoder.send_eof().unwrap();
        receive_and_process_decoded_frames(&mut decoder).unwrap();
    }
}
