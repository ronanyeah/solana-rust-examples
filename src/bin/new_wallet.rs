use solana_sdk::signer::{keypair::Keypair, Signer};

fn main() {
    let pair = Keypair::new();

    println!("Pubkey:\n{}\n", &pair.pubkey().to_string());
    println!("Base58 private key:\n{}\n", &pair.to_base58_string());
    println!("JSON private key:\n{:?}", &pair.to_bytes());
}
