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

pub fn write_chunks() {
    //TODO Chunk code here
}

pub fn write_pixels<'a>(
    screen_data: &'a mut Vec<u8>,
    height: &u8,
    width: &u8,
    buffer: &mut Vec<u8>,
) {
    // Writing stream of image bytes
    buffer.append(screen_data);
}
