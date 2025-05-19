use aes::Aes256;
use block_modes::{BlockMode, Cbc};
use block_modes::block_padding::Pkcs7;

type Aes256Cbc = Cbc<Aes256, Pkcs7>;

fn encrypt(data: &[u8], key: &[u8], iv: &[u8]) -> Vec<u8> {
    let cipher = Aes256Cbc::new_from_slices(key, iv).unwrap();
    cipher.encrypt_vec(data)
}

fn decrypt(encrypted_data: &[u8], key: &[u8], iv: &[u8]) -> Vec<u8> {
    let cipher = Aes256Cbc::new_from_slices(key, iv).unwrap();
    let decrypted_data = cipher.decrypt_vec(encrypted_data).unwrap();

    decrypted_data
}

fn main() {
    let key = b"RfLUeqOwdht2xdqHLlF5H5O2svuQ8dYy"; 
    let iv = b"fIJLithCSy65Ho32"; 
    let data = b"this message will be encrypted";
    let encrypted_data = encrypt(data, key, iv);
    let decrypted_data = decrypt(&encrypted_data, key, iv);
    
    let decrypted_data = String::from_utf8_lossy(&decrypted_data);

    println!("Encrypted data: {:?}", encrypted_data);
    println!("decrypted data {decrypted_data}")


}
