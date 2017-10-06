extern crate crypto_tools_lib;

fn main() {
    use crypto_tools_lib::{bin_to_base64, hexstr_to_base64};

    println!("{}", bin_to_base64("Test"));
    println!("{}", bin_to_base64(&vec![ 0x49, 0x27, 0x6d, 0x20, 0x6b, 0x69, 0x6c, 0x6c,
                                 0x69, 0x6e, 0x67, 0x20, 0x79, 0x6f, 0x75, 0x72,
                                 0x20, 0x62, 0x72, 0x61, 0x69, 0x6e, 0x20, 0x6c,
                                 0x69, 0x6b, 0x65, 0x20, 0x61, 0x20, 0x70, 0x6f,
                                 0x69, 0x73, 0x6f, 0x6e, 0x6f, 0x75, 0x73, 0x20,
                                 0x6d, 0x75, 0x73, 0x68, 0x72, 0x6f, 0x6f, 0x6d,]));
    println!("{}", hexstr_to_base64("49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d"))
}
