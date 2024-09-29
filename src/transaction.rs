use bitcoin::transaction::Transaction;
pub fn get_transaction(txid: &str) -> Result<Transaction, Box<dyn std::error::Error>> {
    let client = reqwest::blocking::Client::new();
    let url = format!("https://blockstream.info/api/tx/{}/hex", txid);
    let response = client.get(&url).send()?;
    let hex = response.text()?;
    let tx = Transaction::from_hex(&hex)?;
    Ok(tx)
}
