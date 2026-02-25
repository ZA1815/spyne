const LOOKUP: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

pub fn encode(bytes: &[u8]) -> String {
    let mut buffer = String::new();
    encode_into(bytes, &mut buffer);
    
    buffer
}

pub fn encode_into(bytes: &[u8], buffer: &mut String) {
    let mut chunks = bytes.chunks_exact(3);
    chunks.by_ref().for_each(|ch| {
        let byte_num = (ch[0] as u32) << 16 | (ch[1] as u32) << 8 | ch[2] as u32;
        let chunk1 = (byte_num >> 18) & 0x3F;
        let chunk2 = (byte_num >> 12) & 0x3F;
        let chunk3 = (byte_num >> 6) & 0x3F;
        let chunk4 = (byte_num) & 0x3F;
        
        [chunk1, chunk2, chunk3, chunk4].iter()
            .for_each(|ch| {
                buffer.push(LOOKUP[*ch as usize] as char);
            });
    });
    let remainder = chunks.remainder();
    match remainder.len() {
        1 => {
            let chunk1 = (remainder[0] >> 2) & 0x3F;
            let chunk2 = (remainder[0] & 0x3) << 4;
            [chunk1, chunk2].iter()
                .for_each(|ch| {
                    buffer.push(LOOKUP[*ch as usize] as char);
                });
            buffer.push_str("==");
        },
        2 => {
            let byte_num = (remainder[0] as u16) << 8 | remainder[1] as u16;
            let chunk1 = (byte_num >> 10) & 0x3F;
            let chunk2 = (byte_num >> 4) & 0x3F;
            let chunk3 = ((byte_num) & 0xF) << 2;
            [chunk1, chunk2, chunk3].iter()
                .for_each(|ch| {
                    buffer.push(LOOKUP[*ch as usize] as char);
                });
            buffer.push('=');
        },
        _ => unreachable!()
    }
}

#[cfg(test)]
mod test {
    use crate::transfer::base64::encode;

    #[test]
    fn test_base64_encoder() {
        let encoded = encode(b"hello");
        assert_eq!(encoded, "aGVsbG8=");
    }
}