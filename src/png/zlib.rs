use adler32::RollingAdler32;

pub fn compress(data: &[u8]) -> Vec<u8> {
    const CHUNK_SIZE: usize = 65530;

    let final_len =
            // header
            2 +
            // every chunk adds 5 bytes [1:type, 4:size].
            (5 * {
                let n = data.len() / CHUNK_SIZE;
                // include an extra chunk when we don't fit exactly into CHUNK_SIZE
                (n + {if data.len() == n * CHUNK_SIZE && data.len() != 0 { 0 } else { 1 }})
            }) +
            // data
            data.len() +
            // crc
            4
        ;

    let mut raw_data = Vec::with_capacity(final_len);
    // header
    raw_data.extend_from_slice(&[120, 1]);
    let mut pos_curr = 0_usize;
    let mut crc = RollingAdler32::new();
    loop {
        let pos_next = std::cmp::min(data.len(), pos_curr + CHUNK_SIZE);
        let chunk_len = (pos_next - pos_curr) as u32;
        let is_last = pos_next == data.len();
        raw_data.extend_from_slice(&[
            // type
            if is_last { 1 } else { 0 },
            // size
            (chunk_len & 0xff) as u8,
            ((chunk_len >> 8) & 0xff) as u8,
            (0xff - (chunk_len & 0xff)) as u8,
            (0xff - ((chunk_len >> 8) & 0xff)) as u8,
        ]);

        raw_data.extend_from_slice(&data[pos_curr..pos_next]);

        crc.update_buffer(&data[pos_curr..pos_next]);

        if is_last {
            break;
        }
        pos_curr = pos_next;
    }

    raw_data.extend_from_slice(&(crc.hash().to_be_bytes()));

    assert_eq!(final_len, raw_data.len());
    return raw_data;
}
