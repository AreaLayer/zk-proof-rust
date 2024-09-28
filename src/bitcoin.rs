use bitcoin::{key::Secp256k1, Network, PrivateKey, PublicKey, Script};
use reqwest::Client;

fn main() {
    // Generate a new private key
    let secp = Secp256k1::new();
    let private_key = PrivateKey::new(&secp, &mut rand::thread_rng());
    let public_key = PublicKey::from_private_key(&secp, &private_key);
    let address = public_key.to_address(Network::Bitcoin);
}

pub fn get_address(privkey: &PrivateKey, network: Network) -> String {
    let public_key = PublicKey::from_private_key(&Secp256k1::new(), privkey);
    let address = public_key.to_address(network).expect("Failed to create address");
    address.to_string()
}

pub async fn get_balance(address: &str) -> Result<f64, Box<dyn std::error::Error>> {
    let client: Client = reqwest::Client::new();
    let response = client.get(&format!("https://blockchain.info/q/addressbalance/{}", address)).send().await?;
    let balance = response.text().await?.parse::<f64>()?;
    Ok(balance)
}