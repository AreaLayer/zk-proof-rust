use bitcoin::{Network, PrivateKey, PublicKey, Script};
use reqwest::Client;

fn main() {
    // Generate a new private key
    let secp = Secp256k1::new();
    let private_key = PrivateKey::new(&secp);
    let public_key = PublicKey::from_private_key(&secp, &private_key);
    let address = public_key.address(Network::Bitcoin).unwrap();
}
pub fn get_address(privkey: &PrivateKey, network: Network) -> String {
    let public_key = PublicKey::from_private_key(&Secp256k1::new(), privkey);
    let script = Script::new_p2pkh(&public_key, network);
    let address = script.address(network).unwrap();
    address.to_string()
}


pub fn get_balance(address: &str) -> Result<f64, Box<dyn std::error::Error>> {
    let client: Client = reqwest::Client::new();
    let response = client.get(&format!("https://blockchain.info/q/addressbalance/{}", address)).send()?;
    let balance = response.text()?.parse::<f64>()?;
    Ok(balance)
}