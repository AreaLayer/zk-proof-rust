use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "zk-coinjoin-cli")]
#[command(about = "ZK Proof CoinJoin CLI", long_about = None)]
pub struct Cli {
    #[arg(long)]
    pub recipient: String,

    #[arg(long)]
    pub amount: u64,

    #[arg(long)]
    pub htlc: String,

    #[arg(long)]
    pub invoice: String,

    #[arg(long)]
    pub electrum: String,

    #[arg(long)]
    pub asset: Option<String>,

    #[arg(long)]
    pub address: Option<String>,

    #[arg(long)]
    pub pool: Option<String>,

    #[arg(long)]
    pub zk: bool,
}

pub fn run() -> Result<(), String> {
    let cli = Cli::parse();

    println!("Running ZK CoinJoin with:");
    println!("Recipient: {}", cli.recipient);
    println!("Amount: {}", cli.amount);
    println!("HTLC: {}", cli.htlc);
    println!("Invoice: {}", cli.invoice);
    println!("Electrum: {}", cli.electrum);
    println!("Use ZK Proof: {}", cli.zk);

    // Example usage of your function from lib
    if cli.zk {
        // Call your zk transaction function
        println!("ZK mode enabled.");
    } else {
        // Call your non-zk function
        println!("Standard mode.");
    }

    Ok(())
}

