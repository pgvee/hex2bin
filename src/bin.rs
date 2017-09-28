extern crate crypto_tools_lib;

fn main() {
    use crypto_tools_lib::bytes_to_base64;

    println!("{}", bytes_to_base64("Test"));
}
