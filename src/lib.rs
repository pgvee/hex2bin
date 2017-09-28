pub fn bytes_to_base64<T: ?Sized + AsRef<[u8]>>(input: &T) -> String {
    const LOWER_SIX_BITS: u32 = 0b11_1111;
    let base64_alphabet = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

    let mut output = String::new();

    let byte_chunks = input.as_ref().chunks(3);
	
	let mut empty_chunk_buffer = vec![0u8; 3];

    for chunk in byte_chunks {
        let mut padding = 0;
        let chunk_buffer = if chunk.len() < 3 {
            for (i, each) in chunk.iter().enumerate() {
                empty_chunk_buffer[i] = *each;
            }
            padding = if chunk.len() == 1 { 2 } else { 1 };
            &empty_chunk_buffer[..]
        } else {
            &chunk[..]
        };

        let temp = ((chunk_buffer[0] as u32) << 16)
                 + ((chunk_buffer[1] as u32) << 8)
                 +  (chunk_buffer[2] as u32);
        
        for x in 0..4 {
            let idx1 = ((temp >> (18 - x*6) & LOWER_SIX_BITS)) as usize;
            let char_to_push: char;
            if padding > 0 && x > 1 {
                char_to_push = b'=' as char;
                padding -= 1;
            } else {
                char_to_push = base64_alphabet[idx1] as char;
            }
            output.push(char_to_push);
        }
    }
    output
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_test_1(){
        assert_eq!(bytes_to_base64("test"), "dGVzdA==");
    }

    #[test]
    fn encode_test_2(){
        assert_eq!(bytes_to_base64(&vec![0x49,0x27,0x6d,0x20,0x6b,0x69,0x6c,
                                         0x6c,0x69,0x6e,0x67,0x20,0x79,0x6f,
                                         0x75,0x72,0x20,0x62,0x72,0x61,0x69,
                                         0x6e,0x20,0x6c,0x69,0x6b,0x65,0x20,
                                         0x61,0x20,0x70,0x6f,0x69,0x73,0x6f,
                                         0x6e,0x6f,0x75,0x73,0x20,0x6d,0x75,
                                         0x73,0x68,0x72,0x6f,0x6f,0x6d]),
            "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t");
    }
}



