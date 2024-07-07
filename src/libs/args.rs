use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    pub image: String,

    #[arg(long, default_value_t = false)]
    pub invert: bool,

    #[arg(long, default_value_t = false)]
    pub colorful: bool,

    #[arg(long, default_value_t = 80)]
    pub width: usize,

    #[arg(long, default_value_t = 25)]
    pub height: usize,

    #[arg(long, default_value_t = 2)]
    pub pixel: usize,

    #[arg(long, default_value_t = false)]
    pub noresize: bool,

    #[arg(long, default_value_t = false)]
    pub matrix: bool,
}
