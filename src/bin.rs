extern crate crypto_tools_lib;

fn main() {
    use crypto_tools_lib::hex_to_base64;

    println!("{}", hex_to_base64("Test"));
}
