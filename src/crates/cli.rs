use clap::{Arg, Command};
use reqwest::blocking::Client;
use zk_coinjoin_lib::{
    create_and_broadcast_transaction,
    create_and_broadcast_transaction_with_zk,
};

pub fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

pub fn run() -> Result<(), String> {
    let matches = Command::new("zk-coinjoin")
        .version("0.1.0")
        .author("Your Name <you@example.com>")
        .about("ZK CoinJoin CLI")
        .arg(Arg::new("recipient")
            .short('r')
            .long("recipient")
            .takes_value(true)
            .required(true)
            .help("Recipient address"))
        .arg(Arg::new("amount")
            .short('a')
            .long("amount")
            .takes_value(true)
            .required(true)
            .help("Amount in sats"))
        .arg(Arg::new("zk")
            .long("zk")
            .takes_value(false)
            .help("Enable ZK proof mode"))
        .arg(Arg::new("taproot")
            .long("taproot")
            .takes_value(false)
            .help("Use Taproot outputs"))
        .arg(Arg::new("htlc")
            .long("htlc")
            .takes_value(true)
            .required(true)
            .help("HTLC string"))
        .arg(Arg::new("invoice")
            .long("invoice")
            .takes_value(true)
            .required(true)
            .help("Lightning invoice"))
        .arg(Arg::new("electrum")
            .long("electrum")
            .takes_value(true)
            .required(true)
            .help("Electrum client URL"))
        .arg(Arg::new("hex")
            .long("hex")
            .takes_value(true)
            .required(false)
            .help("Hex-encoded data"))
        .arg(Arg::new("asset")
            .long("asset")
            .takes_value(true)
            .required(false)
            .help("L-BTC asset ID"))
        .arg(Arg::new("address")
            .long("address")
            .takes_value(true)
            .required(false)
            .help("Sender Liquid address"))
        .arg(Arg::new("pool")
            .long("pool")
            .takes_value(true)
            .required(false)
            .help("Pool address"))
        .get_matches();

    let recipient = matches.value_of("recipient").unwrap();
    let amount: u64 = matches.value_of("amount").unwrap().parse().map_err(|e| e.to_string())?;
    let htlc = matches.value_of("htlc").unwrap();
    let invoice = matches.value_of("invoice").unwrap();
    let electrum = matches.value_of("electrum").unwrap();
    let is_taproot = matches.is_present("taproot");
    let use_zk = matches.is_present("zk");
    let hex = matches.value_of("hex").unwrap_or("");
    let asset_id = matches.value_of("asset").unwrap_or("");
    let address = matches.value_of("address").unwrap_or("");
    let pool = matches.value_of("pool").unwrap_or("");

    let client = Client::new();

    if use_zk {
        create_and_broadcast_transaction_with_zk(
            &client,
            is_taproot,
            recipient,
            amount,
            invoice,
            htlc,
            electrum,
        )
    } else {
        create_and_broadcast_transaction(
            &client,
            is_taproot,
            recipient,
            amount,
            htlc,
            invoice,
            electrum,
            hex,
            asset_id,
            address,
            pool,
        )
    }
} 
