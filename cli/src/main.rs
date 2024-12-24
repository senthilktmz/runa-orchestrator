
use crate::cipher_item::client_encrypt_and_server_decrypt_test;
use runautils::cipher_item;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    client_encrypt_and_server_decrypt_test()?;
    Ok(())
}

//