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

pub fn write_chunks(buffer: &mut Vec<u8>) {
    write_chunk(buffer, "IHDR", &mut vec![]);
}

pub fn write_idat<'a>(screen_data: &'a mut Vec<u8>, height: &u8, width: &u8, buffer: &mut Vec<u8>) {
    // Writing stream of image bytes
    buffer.append(screen_data);
}

pub fn write_iend() {
    //TODO add IEND chunk
}

fn write_chunk(buffer: &mut Vec<u8>, kind: &str, data: &mut Vec<u8>) {
    let type_bytes = kind.as_bytes();

    buffer.extend_from_slice(&(data.len() as u32).to_be_bytes());
    buffer.extend_from_slice(type_bytes);
    buffer.append(data);
}
