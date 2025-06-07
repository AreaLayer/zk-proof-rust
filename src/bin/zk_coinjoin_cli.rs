// src/bin/zk_cli.rs

// tokio = { version = "1.0", features = ["full"] }// tokio = { version = "1.0", features = ["full"] }

#[cfg(feature = "cli")]
use zk_coinjoin_lib::cli::run;

#[cfg(feature = "cli")]
fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

#[cfg(not(feature = "cli"))]
fn main() {
    eprintln!("❌ The `cli` feature is disabled. Please run with:");
    eprintln!("   cargo run --bin zk_cli --features cli");
    std::process::exit(1);
}
