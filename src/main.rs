use clap::Parser;

mod libs;

fn main() {
    eprintln!("ASCII Generator\n----------------");

    let args = libs::args::Args::parse();

    match args.command {
        Some(libs::args::Subcommands::Gen {
            image,
            invert,
            colorful,
            width,
            height,
            pixel,
            noresize,
            matrix,
            nofill,
            output,
            lightmap,
        }) => {
            libs::gen_handler::generator(
                image, invert, colorful, width, height, pixel, noresize, matrix, nofill, output,
                lightmap,
            );
        }
        Some(libs::args::Subcommands::Cam {
            invert,
            colorful,
            width,
            height,
            pixel,
            noresize,
            matrix,
            nofill,
            lightmap,
        }) => {
            libs::cam_handler::camera(
                invert, colorful, width, height, pixel, noresize, matrix, nofill, lightmap,
            );
        }
        Some(libs::args::Subcommands::Vid {
            input,
            invert,
            colorful,
            width,
            height,
            pixel,
            noresize,
            matrix,
            nofill,
            lightmap,
        }) => {
            libs::vid_handler::video(
                input, invert, colorful, width, height, pixel, noresize, matrix, nofill, lightmap,
            );
        }
        None => {
            eprintln!("No subcommand provided. Available subcommands: `gen`, `cam`, and `vid`");
            std::process::exit(1);
        }
    }
}
