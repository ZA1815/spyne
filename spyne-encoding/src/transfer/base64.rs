static LOOKUP: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
static REVERSE_LOOKUP: [u8; 128] = reverse_lookup();
const fn reverse_lookup() -> [u8; 128] {
    let mut rl: [u8; 128] = [0xFF; 128];
    let mut idx: usize = 0;
    while idx < 64 {
        rl[LOOKUP[idx] as usize] = idx as u8;
        idx += 1;
    }
    
    rl
}

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

pub fn decode(string: String) -> Vec<u8> {
    let mut buffer = Vec::new();
    decode_into(string, &mut buffer);
    
    buffer
}

pub fn decode_into(string: String, buffer: &mut Vec<u8>) {
    let string_bytes = string.as_bytes();
    let len = string_bytes.len();
    let mut chunks = string_bytes[..len - 4].chunks_exact(4);
    chunks.by_ref().for_each(|ch| {
        let chunk1 = REVERSE_LOOKUP[ch[0] as usize];
        let chunk2 = REVERSE_LOOKUP[ch[1] as usize];
        let chunk3 = REVERSE_LOOKUP[ch[2] as usize];
        let chunk4 = REVERSE_LOOKUP[ch[3] as usize];
        let byte_num = (chunk1 as u32) << 18 | (chunk2 as u32) << 12 | (chunk3 as u32) << 6 | chunk4 as u32;
        let byte1 = ((byte_num >> 16) & 0xFF) as u8;
        let byte2 = ((byte_num >> 8) & 0xFF) as u8;
        let byte3 = (byte_num & 0xFF) as u8;
        buffer.extend([byte1, byte2, byte3]);
    });
    let last_chunk = &string_bytes[len - 4..len];
    if last_chunk[2] == b'=' && last_chunk[3] == b'=' {
        let chunk1 = REVERSE_LOOKUP[last_chunk[0] as usize];
        let chunk2 = REVERSE_LOOKUP[last_chunk[1] as usize];
        let byte_num = ((chunk1 as u32) << 6 | chunk2 as u32) >> 4;
        let byte1 = (byte_num & 0xFF) as u8;
        buffer.push(byte1);
    }
    else if last_chunk[3] == b'=' {
        let chunk1 = REVERSE_LOOKUP[last_chunk[0] as usize];
        let chunk2 = REVERSE_LOOKUP[last_chunk[1] as usize];
        let chunk3 = REVERSE_LOOKUP[last_chunk[2] as usize];
        let byte_num = ((chunk1 as u32) << 12 | (chunk2 as u32) << 6 | chunk3 as u32) >> 2;
        let byte1 = ((byte_num >> 8) & 0xFF) as u8;
        let byte2 = (byte_num & 0xFF) as u8;
        buffer.extend([byte1, byte2]);
    }
    else {
        let chunk1 = REVERSE_LOOKUP[last_chunk[0] as usize];
        let chunk2 = REVERSE_LOOKUP[last_chunk[1] as usize];
        let chunk3 = REVERSE_LOOKUP[last_chunk[2] as usize];
        let chunk4 = REVERSE_LOOKUP[last_chunk[3] as usize];
        let byte_num = (chunk1 as u32) << 18 | (chunk2 as u32) << 12 | (chunk3 as u32) << 6 | chunk4 as u32;
        let byte1 = ((byte_num >> 16) & 0xFF) as u8;
        let byte2 = ((byte_num >> 8) & 0xFF) as u8;
        let byte3 = (byte_num & 0xFF) as u8;
        buffer.extend([byte1, byte2, byte3]);
    }
}

#[cfg(test)]
mod test {
    use crate::transfer::base64::{decode, encode};

    #[test]
    fn test_base64_encoder() {
        let encoded = encode(b"hello");
        assert_eq!(encoded, "aGVsbG8=");
        let decoded = decode("aGVsbG8=".to_string());
        assert_eq!(decoded, b"hello");
    }
}