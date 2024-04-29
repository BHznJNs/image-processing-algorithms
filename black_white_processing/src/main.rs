#![allow(clippy::needless_return)]

use image::{
    DynamicImage,
    ImageBuffer,
    io::Reader as ImageReader,
    GenericImageView,
    Rgb, Pixel,
};

const IMAGE_PATH: &str = "./source_image/lena.png";
const OUTPUT_PATH: &str = "./result-image/lena.png";

fn H(img: &DynamicImage, intensity: u32) {
    // 
}

fn main() {
    let source_img = ImageReader::open(IMAGE_PATH)
        .unwrap()
        .decode()
        .unwrap();

    let (width, height) = source_img.dimensions();
    let mut img_buf: ImageBuffer<Rgb<u32>, Vec<_>> = ImageBuffer::new(width, height);
    for (x, y, pixel) in img_buf.enumerate_pixels_mut() {
        let source_pixel = source_img.get_pixel(x, y);
        // 
    }
    
    // for x in 0..width {
    //     for y in 0..height {
    //         // let pixel = img.get_pixel(x, y);
    //         // 
    //         // pixel.
    //     }
    // }
}
