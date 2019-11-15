use crc::{crc32, Hasher32};

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
    write_ihdr(buffer, width, height, 32u8, 6u8, 0u8, 0u8, 0u8);
    write_idat(buffer, screen_data);
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
    let mut ihdr_buff = Vec::with_capacity(13);
    ihdr_buff.extend_from_slice(&height.to_be_bytes());
    ihdr_buff.extend_from_slice(&width.to_be_bytes());
    ihdr_buff.push(bit_depth);
    ihdr_buff.push(color_type);
    ihdr_buff.push(compression_method);
    ihdr_buff.push(filter_method);
    ihdr_buff.push(interlace_method);
    write_chunk(buffer, "IHDR", &mut ihdr_buff);
}

pub fn write_idat<'a>(buffer: &mut Vec<u8>, screen_data: &'a [u8]) {
    let mut screen_u32: Vec<u8> = screen_data.iter().fold(vec![], |mut bytes, x| {
        let x_as_u32 = (*x as u32).to_be_bytes();
        bytes.extend_from_slice(&x_as_u32);
        bytes
    });

    write_chunk(buffer, "IDAT", &mut screen_u32)
}

pub fn write_iend(buffer: &mut Vec<u8>) {
    write_chunk(buffer, "IEND", &mut vec![]);
}

fn write_chunk(buffer: &mut Vec<u8>, kind: &str, data: &mut Vec<u8>) {
    let type_bytes = kind.as_bytes();

    buffer.extend_from_slice(&(data.len() as u32).to_be_bytes());
    buffer.extend_from_slice(type_bytes);
    buffer.append(data);

    let mut digest = crc32::Digest::new(crc32::IEEE);
    digest.write(type_bytes);
    digest.write(data.as_slice());
    buffer.extend_from_slice(&digest.sum32().to_be_bytes());
}
