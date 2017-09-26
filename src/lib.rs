pub fn hex_to_base64<T: ?Sized + AsRef<[u8]>>(input: &T) -> String {
    const LOWER_SIX_BITS: u32 = 0b111111;
    let base64_alphabet = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/=";

    let mut output = String::new();

    let byte_chunks = input.as_ref().chunks(3);

    let mut empty_chunk_buffer = vec!(0u8,0u8,0u8);

    for chunk in byte_chunks {
        let mut chunk_buffer = chunk.clone();
        if chunk.len() > 3 {
            chunk_buffer = chunk.iter().collect::<_>();
            for (i, each) in (*chunk).iter().enumerate() {
                chunk_buffer[i] = each.clone();
            }
        }

        let temp = ((chunk_buffer[0] as u32) << 16)
                 + ((chunk_buffer[1] as u32) << 8)
                 +  (chunk_buffer[2] as u32);
        let idx1 = ((temp >> 18) & LOWER_SIX_BITS) as usize;
        output.push((base64_alphabet[idx1].clone() as char));
    }
    output
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_test_1(){
        hex_to_base64("test");
        // assert!("SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t", );
    }
}



