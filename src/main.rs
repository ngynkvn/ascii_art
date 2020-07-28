use std::{path::PathBuf, ops::Index};
use image::{self, imageops::FilterType};
use structopt::StructOpt;
#[derive(Debug, StructOpt)]
#[structopt(name = "ascii_art", about = "Create an ascii image on your terminal.")]
struct Opt {
    /// Path to the image.
    #[structopt(parse(from_os_str))]
    path: PathBuf,
    /// Width of the image.
    #[structopt(default_value = "50")]
    width: u32,
    /// Height of the image.
    #[structopt(default_value = "50")]
    height: u32,
    /// Invert
    #[structopt(long, short)]
    invert: bool,
}

const BRIGHTNESS: &[u8] = "`^\",:;Il!i~+_-?][}{1)(|\\/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B@$".as_bytes();
const BRIGHTNESS_LEN: f32 = (BRIGHTNESS.len() - 1) as f32;
fn map_to_text(luminosity: u8) -> char {
    return BRIGHTNESS[((BRIGHTNESS_LEN / 255.0) * luminosity as f32) as usize ] as char;
}

fn get_luminosity<T: Index<usize, Output=u8> + ?Sized>(pixel: &T) -> u8 {
    return ((u32::from(pixel[0]) + u32::from(pixel[1]) + u32::from(pixel[2])) / 3) as u8;
}

fn invert(value: u8) -> u8 {
    255 - value
}

fn main() {
    let opt = Opt::from_args();
    let path = std::path::Path::new(&opt.path);
    let image = image::open(path).expect("Unable to open the file path.");
    let image = image.resize(opt.width, opt.height, FilterType::Nearest);
    let image = image.to_rgb();
    image.pixels()
        .map(get_luminosity)
        .map(|x| if opt.invert {
                invert(x)
            } else {
                x
            })
        .map(map_to_text)
        .enumerate()
        .for_each(|(i, chr)| {
            if i as u32 % image.width() == 0 {
                println!();
            } else {
                print!("{}{}{}", chr, chr, chr);
            }
        });
}
