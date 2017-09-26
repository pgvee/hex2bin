pub fn hex_to_base64<T: ?Sized + AsRef<[u8]>>(input: &T) -> String {
    const LOWER_SIX_BITS: u32 = 0b111111;
    let base64_alphabet = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/=";

    let mut output = String::new();

    let mut bytes = input.as_ref().to_owned();
    let byte_chunks = bytes.chunks_mut(3);

    let mut empty_chunk_buffer = vec![0, 0, 0];

    for chunk in byte_chunks {
        let chunk_buffer = if chunk.len() < 3 {
            for (i, each) in chunk.iter().enumerate() {
                empty_chunk_buffer[i] = *each;
            }
            &empty_chunk_buffer[..]
        } else {
            &chunk[..]
        };

        let temp = ((chunk_buffer[0] as u32) << 16)
                 + ((chunk_buffer[1] as u32) << 8)
                 +  (chunk_buffer[2] as u32);
        let idx1 = ((temp >> 18) & LOWER_SIX_BITS) as usize;
        output.push(base64_alphabet[idx1] as char);
    }
    output
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_test_1(){
        assert_eq!(hex_to_base64("test"), "dGVzdAo=");
    }
}



