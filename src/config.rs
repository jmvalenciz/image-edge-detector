use clap::Parser;


// Program to get border of an image
#[derive(Parser, Debug)]
#[clap(name = "Image Edge Detector")]
#[clap(author = "Juan Manuel Valencia Zapata <jmvalenciz@protonmail.com>")]
#[clap(version = "1.0")]
#[clap(about = "Program to get border of an image", long_about = None)]
#[clap(author, version, about, long_about = None)]
pub struct Config{
    // Input image
    #[clap(short, long)]
    pub input: String,
    // Output image
    #[clap(short, long)]
    pub output: String,
    // Apply color
    #[clap(short, long, parse(try_from_str), default_value_t = false)]
    pub color: bool,
    // Blur level
    #[clap(short, long, default_value_t = 0.0)]
    pub blur: f32,
    // check noise gate to remove pixels with low intensity
    #[clap(short, long, default_value_t = 0.0)]
    pub noise_gate: f32

}
