mod config;

#[tokio::main]
async fn main() {
    let _config = config::Config::load();
    // Phase 2 will use _config for WebSocket authentication
    println!("CSGORoll Sniper ready.");
}
