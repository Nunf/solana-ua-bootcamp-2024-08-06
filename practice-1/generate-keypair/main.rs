use solana_sdk::signature::Keypair;

fn main() {
    let keypair = Keypair::new();

    println!("✅ Keypair generated");
    println!("Public key: {}", keypair.pubkey());
    println!("Secret key: {:?}", keypair.to_bytes());
}