use bitcoin::transaction::Transaction;
use spawn_zk_snarks::keygen::KeyPair;
use spawn_zk_snarks::proof::generate_proof;
use spawn_zk_snarks::utils::random_witness;
use spawn_zk_snarks::utils::hash_witness;


pub fn get_transaction(txid: &str) -> Result<Transaction, Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();
    let url = format!("https://blockstream.info/api/tx/{}/hex", txid);
    let response = client.get(&url).send()?;
    let hex = response.text()?;
    let tx = Transaction::from_hex(&hex)?;
    Ok(tx)
}
