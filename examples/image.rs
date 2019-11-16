extern crate scrap;

use scrap::{Capturer, Display};
use std::fs::File;
use std::io::prelude::*;

use image_encoder::png;

fn main() {
    let display = Display::primary().expect("Couldn't find primary display.");
    let mut capturer = Capturer::new(display).expect("Couldn't begin capture.");
    let (width, height) = (capturer.width(), capturer.height());

    let frame = capturer.frame().unwrap();

    let mut bitflipped = Vec::with_capacity(width * height * 4);
    let stride = frame.len() / height;

    for y in 0..height {
        for x in 0..width {
            let i = stride * y + 4 * x;
            bitflipped.extend_from_slice(&[frame[i + 2], frame[i + 1], frame[i], 255]);
        }
    }

    let mut file = File::create("output.png").unwrap();
    png::write_header(&mut file).unwrap();
    png::write_chunks(
        &mut file,
        bitflipped.as_slice(),
        width as u32,
        height as u32,
    )
    .unwrap();
    file.flush().unwrap();
}
