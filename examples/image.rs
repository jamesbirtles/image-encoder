extern crate scrap;

use bmp;
use scrap::{Capturer, Display};
use std::fs::File;
use std::io::prelude::*;
use std::io::ErrorKind::WouldBlock;
use std::ops::Deref;
use std::thread;
use std::time::Duration;

use image_encoder;

fn main() {
    // let display = Display::primary().expect("Couldn't find primary display.");
    // let mut capturer = Capturer::new(display).expect("Couldn't begin capture.");
    // let (w, h) = (capturer.width(), capturer.height());

    // let buffer = capturer.frame().unwrap();

    let img = bmp::open("pixel.bmp").unwrap();

    let mut buffer = vec![];
    for (x, y) in img.coordinates() {
        let pixel = img.get_pixel(x, y);
        buffer.push(pixel.r);
        buffer.push(pixel.g);
        buffer.push(pixel.b);
        // buffer.push(0xFF);
    }
    // bitmap

    let mut buf = vec![];
    image_encoder::write_header(&mut buf);
    image_encoder::write_chunks(&mut buf, buffer.deref(), 1u32, 1u32);

    let mut file = File::create("output.png").unwrap();
    file.write_all(buf.as_slice()).unwrap();
    file.flush().unwrap();
}
