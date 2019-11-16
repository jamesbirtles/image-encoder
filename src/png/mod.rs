mod zlib;

use crc::{crc32, Hasher32};
use std::io;
use zlib::compress;

pub struct Image<'a> {
    pub width: u32,
    pub height: u32,
    data: &'a [u8],
}

impl<'a> Image<'a> {
    pub fn new(data: &'a [u8], width: u32, height: u32) -> Image<'a> {
        Image {
            data,
            width,
            height,
        }
    }

    pub fn encode_into<W: io::Write>(&self, buffer: &mut W) -> io::Result<()> {
        write_header(buffer)?;
        write_chunks(buffer, self.data, self.width, self.height)
    }
}

pub fn write_header<W: io::Write>(buffer: &mut W) -> io::Result<()> {
    buffer.write_all(b"\x89PNG\r\n\x1A\n")
}

pub fn write_chunks<W: io::Write>(
    buffer: &mut W,
    screen_data: &[u8],
    width: u32,
    height: u32,
) -> io::Result<()> {
    write_ihdr(buffer, width, height, 8u8, 6u8, 0u8, 0u8, 0u8)?;
    write_idat(buffer, screen_data, width, height)?;
    write_iend(buffer)?;

    Ok(())
}

fn write_ihdr<W: io::Write>(
    buffer: &mut W,
    width: u32,
    height: u32,
    bit_depth: u8,
    color_type: u8,
    compression_method: u8,
    filter_method: u8,
    interlace_method: u8,
) -> io::Result<()> {
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
    write_chunk(buffer, b"IHDR", &mut data)
}

fn write_idat<W: io::Write>(
    buffer: &mut W,
    screen_data: &[u8],
    width: u32,
    height: u32,
) -> io::Result<()> {
    let mut filtered = vec![];
    for y in 0..height {
        let start = (y * width * 4) as usize;
        let end = start + (width as usize) * 4;
        filtered.push(0);
        let data = &screen_data[start..end];
        filtered.extend_from_slice(data);
    }
    let compressed = compress(&filtered);

    write_chunk(buffer, b"IDAT", &compressed)
}

fn write_iend<W: io::Write>(buffer: &mut W) -> io::Result<()> {
    write_chunk(buffer, b"IEND", &[])
}

fn write_chunk<W: io::Write>(buffer: &mut W, kind: &[u8; 4], data: &[u8]) -> io::Result<()> {
    buffer.write_all(&(data.len() as u32).to_be_bytes())?;
    buffer.write_all(kind)?;
    buffer.write_all(data)?;

    let mut digest = crc32::Digest::new(crc32::IEEE);
    digest.write(kind);
    digest.write(data);
    buffer.write_all(&digest.sum32().to_be_bytes())?;

    Ok(())
}
