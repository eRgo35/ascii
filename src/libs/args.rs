use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Option<Subcommands>,
}

#[derive(Subcommand, Debug)]
pub enum Subcommands {
    Gen {
        #[arg(short, long, default_value_t = String::from(""))]
        image: String,

        #[arg(long, default_value_t = false)]
        invert: bool,

        #[arg(long, default_value_t = false)]
        colorful: bool,

        #[arg(long, default_value_t = 80)]
        width: usize,

        #[arg(long, default_value_t = 25)]
        height: usize,

        #[arg(long, default_value_t = 2)]
        pixel: usize,

        #[arg(long, default_value_t = false)]
        noresize: bool,

        #[arg(long, default_value_t = false)]
        matrix: bool,

        #[arg(long, default_value_t = false)]
        nofill: bool,

        #[arg(long, default_value_t = String::from(""))]
        output: String,

        #[arg(long, default_value_t = String::from(""))]
        lightmap: String,
    },

    Cam {
        #[arg(long, default_value_t = false)]
        invert: bool,

        #[arg(long, default_value_t = false)]
        colorful: bool,

        #[arg(long, default_value_t = 80)]
        width: usize,

        #[arg(long, default_value_t = 25)]
        height: usize,

        #[arg(long, default_value_t = 2)]
        pixel: usize,

        #[arg(long, default_value_t = false)]
        noresize: bool,

        #[arg(long, default_value_t = false)]
        matrix: bool,

        #[arg(long, default_value_t = false)]
        nofill: bool,

        #[arg(long, default_value_t = String::from(""))]
        lightmap: String,
    },
}