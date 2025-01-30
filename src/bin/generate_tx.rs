use hex;
use rand::RngCore;

fn main() {
    // Generate random 64-byte transaction
    let mut tx = [0u8; 64];
    rand::thread_rng().fill_bytes(&mut tx);
    
    // Convert to hex string with /tx prefix
    println!("/tx {}", hex::encode(tx));
}