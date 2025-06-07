// src/bin/zk_cli.rs

// tokio = { version = "1.0", features = ["full"] }// tokio = { version = "1.0", features = ["full"] }


#[cfg(not(feature = "cli"))]
fn main() {
    eprintln!("❌ The `cli` feature is disabled. Please run with:");
    eprintln!("   cargo run --bin zk_cli --features cli");
    std::process::exit(1);
}
