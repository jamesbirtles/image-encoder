extern crate scrap;

use scrap::{Capturer, Display};
use std::fs::File;
use std::io::prelude::*;
use std::io::ErrorKind::WouldBlock;
use std::thread;
use std::time::Duration;

use image_encoder;

fn main() {
    // capture_screen();

    let mut buf = vec![];
    image_encoder::write_header(&mut buf);
    image_encoder::write_chunks(&mut buf);

    let mut file = File::create("output.png").unwrap();
    file.write_all(buf.as_slice()).unwrap();
    file.flush().unwrap();

    println!("{:#x?}", buf);
}

fn capture_screen() {
    let one_second = Duration::new(1, 0);
    let one_frame = one_second / 60;

    let display = Display::primary().expect("Couldn't find primary display.");
    let mut capturer = Capturer::new(display).expect("Couldn't begin capture.");
    let (w, h) = (capturer.width(), capturer.height());

    loop {
        // Wait until there's a frame.

        let buffer = match capturer.frame() {
            Ok(buffer) => buffer,
            Err(error) => {
                if error.kind() == WouldBlock {
                    // Keep spinning.
                    thread::sleep(one_frame);
                    continue;
                } else {
                    panic!("Error: {}", error);
                }
            }
        };
    }
}
