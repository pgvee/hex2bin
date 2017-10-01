/// Returns a String of ASCII characters representing the Base64
/// encoding of the input array.
///
/// # Arguments
/// * `input` - A reference to the input data
///
/// Base 64 encoder works on a reference to raw bytes.
/// It breaks the byte array up in to 3-byte chunks, then moves
/// a 6-bit wide window along the chunk to produce an index
/// for the base64 alphabet.
///
/// The function will use a small temporary buffer for the case
/// where the chuck is not a full 3-bytes and will pad the
/// output string with '=' if the input is not divisible by 3.
///
/// This function does no pre-processing on input arrays
// trait b64ops {
//     fn base64_encode<T: ?Sized + AsRef<[u8]>>(input: &T);
// }

pub fn base64_encode<T: AsRef<[u8]> + ?Sized>(input: &T) -> String {
    const LOWER_SIX_BITS: u32 = 0b11_1111;
    let base64_alphabet = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

    let mut working_buffer = vec![0u8; 3];
    let mut padding = 0; // to determine how many '=' we need
    let mut output = String::new();

    let byte_chunks = input.as_ref().chunks(3);

    for chunk in byte_chunks {
        let chunk_buffer = if chunk.len() < 3 {
            padding = if chunk.len() == 1 { 2 } else { 1 }; // used later
            for (i, each) in chunk.iter().enumerate() {
                working_buffer[i] = *each;
            }
            &working_buffer
        } else {
            chunk
        };

        let temp = ((chunk_buffer[0] as u32) << 16) + ((chunk_buffer[1] as u32) << 8)
            + (chunk_buffer[2] as u32);

        for x in 0..4 {
            let idx1 = (temp >> (18 - x * 6) & LOWER_SIX_BITS) as usize;
            let char_to_push: char;
            if padding > 0 && x > 1 {
                // add up to 2 '='s
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
    fn encode_test_1() {
        assert_eq!(base64_encode("test"), "dGVzdA==");
    }

    // #[rustfmt_skip]
    #[test]
    fn encode_test_2() {
        assert_eq!(
            base64_encode(&vec![ 0x49, 0x27, 0x6d, 0x20, 0x6b, 0x69, 0x6c, 0x6c,
                                 0x69, 0x6e, 0x67, 0x20, 0x79, 0x6f, 0x75, 0x72,
                                 0x20, 0x62, 0x72, 0x61, 0x69, 0x6e, 0x20, 0x6c,
                                 0x69, 0x6b, 0x65, 0x20, 0x61, 0x20, 0x70, 0x6f,
                                 0x69, 0x73, 0x6f, 0x6e, 0x6f, 0x75, 0x73, 0x20,
                                 0x6d, 0x75, 0x73, 0x68, 0x72, 0x6f, 0x6f, 0x6d]),
            "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t"
        );
    }
}
