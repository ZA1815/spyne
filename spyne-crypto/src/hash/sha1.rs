use std::iter::repeat;

pub fn hash(input: &[u8]) -> [u8; 20]{
    let mut padded: Vec<u8> = Vec::with_capacity(input.len().div_ceil(64));
    let rem = input.chunks_exact(64).remainder();
    if rem.len() <= 55 {
        padded.extend(rem);
        padded.push(0x80);
        padded.extend(repeat(0x00).take(55 - rem.len()));
        padded.extend((input.len() as u64 * 8).to_be_bytes());
    }
    else {
        padded.extend(rem);
        padded.push(0x80);
        padded.extend(repeat(0x00).take(63 - rem.len()));
        padded.extend(repeat(0x00).take(56));
        padded.extend((input.len() as u64 * 8).to_be_bytes());
    }
}