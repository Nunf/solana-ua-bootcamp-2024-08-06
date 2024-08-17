use solana_sdk::{signature::Keypair, signer::Signer};

fn main() {
    let private_key = if let Ok(pk) = dotenv::var("PRIVATE_KEY"){
        string_to_u8_array(pk.as_str())
    } else {
        panic!("invalid private key");
    };

    let keypair = Keypair::from_bytes(&private_key).unwrap();

    println!("{}", keypair.pubkey());
}

fn string_to_u8_array(input: &str) -> [u8; 64] {
    input
        .split(',')
        .map(|x| x.trim().parse::<u8>().unwrap())
        .collect::<Vec<_>>()
        .try_into()
        .unwrap()
}