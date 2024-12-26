use clap::Parser;
use image::{imageops::FilterType, DynamicImage, ImageBuffer, Luma, Pixel};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    image_path: String,

    #[arg(short, long, default_value_t = 8)]
    descale_rate: u32,
}

fn gray_scale_image(input_image: DynamicImage, c: f32) -> ImageBuffer<Luma<u8>, Vec<u8>> {
    input_image.adjust_contrast(c).to_luma8()
}

fn resize_image(input_image: DynamicImage, h: u32, w: u32) -> DynamicImage {
    input_image.resize(w, h, FilterType::Lanczos3)
}

fn print_assci_on_intensity(intensity: u8) {
    match intensity {
        90..=255 => print!(" "),
        80..=89 => print!("."),
        60..=79 => print!("+"),
        40..=59 => print!("*"),
        20..=39 => print!("&"),
        1..=19 => print!("/"),
        0 => print!("@"),
    }
}

fn main() {
    let args = Args::parse();

    let img = image::open(args.image_path).unwrap();
    let down_scaled_height = (img.height() * args.descale_rate) / 100;
    let down_scaled_width = (img.width() * args.descale_rate) / 100;

    let resized_img = resize_image(img, down_scaled_height, down_scaled_width);
    let gray_scaled_img = gray_scale_image(resized_img, 20.0);

    for y in 0..gray_scaled_img.height() {
        for x in 0..gray_scaled_img.width() {
            let intensity = gray_scaled_img.get_pixel(x, y).channels()[0];
            print_assci_on_intensity(intensity)
        }
        print!("\n");
    }
}
