use solana_sdk::{native_token::LAMPORTS_PER_SOL, signature::Keypair, signer::Signer};
use solana_client::rpc_client::RpcClient;

fn main() {
    let private_key = if let Ok(pk) = dotenv::var("PRIVATE_KEY"){
        string_to_u8_array(pk.as_str())
    } else {
        panic!("invalid private key");
    };

    let keypair = Keypair::from_bytes(&private_key).unwrap();

    println!("{}", keypair.pubkey());

    let rpc = RpcClient::new("https://api.devnet.solana.com");
    let account = rpc.get_account(&keypair.pubkey()).unwrap();
    let balance_in_lamports = account.lamports;
    let balance_in_sol = balance_in_lamports / LAMPORTS_PER_SOL;
    println!("Balance: {}", balance_in_sol);
}

fn string_to_u8_array(input: &str) -> [u8; 64] {
    input
        .split(',')
        .map(|x| x.trim().parse::<u8>().unwrap())
        .collect::<Vec<_>>()
        .try_into()
        .unwrap()
}