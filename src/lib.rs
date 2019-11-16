use crc::{crc32, Hasher32};
use deflate::deflate_bytes;
use deflate::write::ZlibEncoder;
use deflate::Compression;
use std::io::Write;

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
    write_ihdr(buffer, width, height, 8u8, 2u8, 0u8, 0u8, 0u8);
    write_idat(buffer, screen_data, width, height);
    write_iend(buffer);
}

pub fn write_ihdr(
    buffer: &mut Vec<u8>,
    width: u32,
    height: u32,
    bit_depth: u8,
    color_type: u8,
    compression_method: u8,
    filter_method: u8,
    interlace_method: u8,
) {
    let mut ihdr_buff = vec![];
    ihdr_buff.extend_from_slice(&width.to_be_bytes());
    ihdr_buff.extend_from_slice(&height.to_be_bytes());
    ihdr_buff.extend_from_slice(&bit_depth.to_be_bytes());
    ihdr_buff.extend_from_slice(&color_type.to_be_bytes());
    ihdr_buff.extend_from_slice(&compression_method.to_be_bytes());
    ihdr_buff.extend_from_slice(&filter_method.to_be_bytes());
    ihdr_buff.extend_from_slice(&interlace_method.to_be_bytes());
    write_chunk(buffer, "IHDR", &mut ihdr_buff);
}

pub fn write_idat<'a>(buffer: &mut Vec<u8>, screen_data: &'a [u8], width: u32, height: u32) {
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::Fast);

    for y in 0..height {
        let start = (y * width * 3) as usize;
        let end = start + (width as usize) * 3;
        encoder.write_all(&[0]).unwrap();
        encoder.write_all(&screen_data[start..end]).unwrap();
    }

    let mut compressed = encoder.finish().unwrap();

    write_chunk(buffer, "IDAT", &mut compressed);
}

pub fn write_iend(buffer: &mut Vec<u8>) {
    write_chunk(buffer, "IEND", &mut vec![]);
}

fn write_chunk(buffer: &mut Vec<u8>, kind: &str, data: &mut Vec<u8>) {
    let type_bytes = kind.as_bytes();

    buffer.extend_from_slice(&(data.len() as u32).to_be_bytes());
    buffer.extend_from_slice(type_bytes);
    buffer.append(data);

    let mut crc = CRC32(type_bytes, 0, 4, 0);
    crc = CRC32(data.as_slice(), 0, data.len(), crc);

    // let mut digest = crc32::Digest::new(crc32::IEEE);
    // digest.write(type_bytes);
    // digest.write(data.as_slice());
    // buffer.extend_from_slice(&digest.sum32().to_be_bytes());
    buffer.extend_from_slice(&crc.to_be_bytes());
}

static mut CRC_TABLE: Vec<u32> = Vec::new();

fn CRC32(stream: &[u8], offset: usize, length: usize, crc: u32) -> u32 {
    let mut c: u32;
    unsafe {
        if CRC_TABLE.len() == 0 {
            CRC_TABLE = Vec::with_capacity(256);
            for n in 0..255 {
                c = n;
                for k in 0..8 {
                    c = if (c & 1) == 1 {
                        0xEDB88320 ^ ((c >> 1) & 0x7FFFFFFF)
                    } else {
                        ((c >> 1) & 0x7FFFFFFF)
                    }
                }
                CRC_TABLE.push(c);
            }
        }

        c = crc ^ 0xffffffff;
        let end_offset = offset + length;
        for i in offset..end_offset {
            c = CRC_TABLE[((c ^ (stream[i] as u32)) & 255) as usize] ^ ((c >> 8) & 0xFFFFFF);
        }
        c ^ 0xffffffff
    }
}
