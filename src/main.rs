mod config;
mod ws;

#[tokio::main]
async fn main() {
    let config = config::Config::load();

    println!("[main] CSGORoll Sniper starting persistent WebSocket loop...");

    ws::run_with_reconnect(&config, |trade| {
        println!(
            "[trade] id={} markup={:.2}% price={:.2} canJoin={} status={} item=\"{}\" stattrak={}",
            trade.id,
            trade.markup_percent,
            trade.total_value,
            trade.can_join,
            trade.status,
            trade.market_name,
            trade.is_stattrak,
        );
    })
    .await;
}
