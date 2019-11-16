mod zlib;

use crc::{crc32, Hasher32};
use zlib::compress;

pub fn write_header(buffer: &mut Vec<u8>) {
    buffer.push(0x89);

    // PNG
    buffer.push(0x50);
    buffer.push(0x4E);
    buffer.push(0x47);

    // DOS Line Endings
    buffer.push(0x0D);
    buffer.push(0x0A);

    // EOF
    buffer.push(0x1A);

    // Unix line ending
    buffer.push(0x0A);
}

pub fn write_chunks(buffer: &mut Vec<u8>, screen_data: &[u8], width: u32, height: u32) {
    write_ihdr(buffer, width, height, 8u8, 6u8, 0u8, 0u8, 0u8);
    write_idat(buffer, screen_data, width, height);
    write_iend(buffer);
}

fn write_ihdr(
    buffer: &mut Vec<u8>,
    width: u32,
    height: u32,
    bit_depth: u8,
    color_type: u8,
    compression_method: u8,
    filter_method: u8,
    interlace_method: u8,
) {
    let wb = width.to_be_bytes();
    let hb = height.to_be_bytes();
    let mut data = [
        wb[0],
        wb[1],
        wb[2],
        wb[3],
        hb[0],
        hb[1],
        hb[2],
        hb[3],
        bit_depth,
        color_type,
        compression_method,
        filter_method,
        interlace_method,
    ];
    write_chunk(buffer, b"IHDR", &mut data);
}

fn write_idat(buffer: &mut Vec<u8>, screen_data: &[u8], width: u32, height: u32) {
    let mut filtered = vec![];
    for y in 0..height {
        let start = (y * width * 4) as usize;
        let end = start + (width as usize) * 4;
        filtered.push(0);
        let data = &screen_data[start..end];
        filtered.extend_from_slice(data);
    }
    let compressed = compress(&filtered);

    write_chunk(buffer, b"IDAT", &compressed);
}

fn write_iend(buffer: &mut Vec<u8>) {
    write_chunk(buffer, b"IEND", &[]);
}

fn write_chunk(buffer: &mut Vec<u8>, kind: &[u8; 4], data: &[u8]) {
    buffer.extend_from_slice(&(data.len() as u32).to_be_bytes());
    buffer.extend_from_slice(kind);
    buffer.extend_from_slice(data);

    let mut digest = crc32::Digest::new(crc32::IEEE);
    digest.write(kind);
    digest.write(data);
    buffer.extend_from_slice(&digest.sum32().to_be_bytes());
}
