use std::io::{self, Write};

use clap::Parser;
use image::{imageops::FilterType, DynamicImage, GenericImageView, Pixel};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    image_path: String,

    #[arg(short, long, default_value_t = 8)]
    descale_rate: u32,
}

fn gray_scale_pixel(r: u8, g: u8, b: u8) -> u8 {
    (r + g + b) / 3
}

fn resize_image(input_image: DynamicImage, h: u32, w: u32) -> DynamicImage {
    input_image.resize_exact(w, h, FilterType::Lanczos3)
}

fn print_rgb_text<W: Write>(writer: &mut W, r: u8, g: u8, b: u8, text: &str) {
    write!(writer, "\x1b[38;2;{};{};{}m{}\x1b[0m", r, g, b, text).unwrap();
}

fn print_assci_on_intensity(intensity: u8, r: u8, g: u8, b: u8) {
    let mut stdout = io::stdout();
    match intensity {
        90..=255 => print_rgb_text(&mut stdout, r, g, b, " "),
        80..=89 => print_rgb_text(&mut stdout, r, g, b, "."),
        60..=79 => print_rgb_text(&mut stdout, r, g, b, "+"),
        40..=59 => print_rgb_text(&mut stdout, r, g, b, "*"),
        20..=39 => print_rgb_text(&mut stdout, r, g, b, "&"),
        1..=19 => print_rgb_text(&mut stdout, r, g, b, "/"),
        0 => print_rgb_text(&mut stdout, r, g, b, "@"),
    }
}

fn main() {
    let args = Args::parse();

    let img = image::open(args.image_path).unwrap();
    let down_scaled_height = (img.height() * args.descale_rate) / 100;
    let down_scaled_width = (img.width() * args.descale_rate) / 50; // divide by 50 because hight = 2 x width, (in terminal)

    let resized_img = resize_image(img, down_scaled_height, down_scaled_width);

    for y in 0..resized_img.height() {
        for x in 0..resized_img.width() {
            let intensity = gray_scale_pixel(
                resized_img.get_pixel(x, y).channels()[0],
                resized_img.get_pixel(x, y).channels()[1],
                resized_img.get_pixel(x, y).channels()[2],
            );
            print_assci_on_intensity(
                intensity,
                resized_img.get_pixel(x, y).channels()[0],
                resized_img.get_pixel(x, y).channels()[1],
                resized_img.get_pixel(x, y).channels()[2],
            )
        }
        print!("\n");
    }
}
