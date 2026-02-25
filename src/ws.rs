use crate::config::Config;
use serde::Deserialize;
use std::time::Duration;
use tokio_tungstenite::{
    connect_async_tls_with_config,
    tungstenite::{
        client::IntoClientRequest,
        Message,
    },
    Connector,
};
use futures_util::{SinkExt, StreamExt};

// --- Trade types (Phase 3 interface) ---

#[derive(Debug, Clone)]
pub struct Trade {
    pub id: String,
    pub markup_percent: f64,
    pub total_value: f64,
    pub can_join: bool,
    pub status: String,
    // From tradeItems[0]
    pub market_name: String,
    pub brand: String,
    pub skin_name: String,
    pub wear: String,
    pub is_stattrak: bool,
}

// --- Internal deserialization types ---

#[derive(Deserialize, Debug)]
struct WsMessage {
    #[serde(rename = "type")]
    msg_type: String,
    payload: Option<serde_json::Value>,
}

// --- Connection ---

const WS_URL: &str = "wss://router.csgoroll.com/ws";

/// Single connection attempt. Opens WS, completes graphql-transport-ws handshake,
/// then calls on_trade for each received Trade until the connection closes or errors.
/// Returns Ok(()) if connection closed cleanly, Err on protocol/network error.
pub async fn connect_once(
    config: &Config,
    mut on_trade: impl FnMut(Trade),
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // Build the upgrade request with required headers
    let mut request = WS_URL.into_client_request()?;
    let headers = request.headers_mut();
    headers.insert(
        "Sec-WebSocket-Protocol",
        "graphql-transport-ws".parse()?,
    );
    headers.insert(
        "Origin",
        "https://www.csgoroll.com".parse()?,
    );
    headers.insert(
        "Cookie",
        config.csgoroll_session.parse()?,
    );
    headers.insert(
        "User-Agent",
        "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/133.0.0.0 Safari/537.36".parse()?,
    );

    println!("[ws] Connecting to {}...", WS_URL);
    let (mut ws, _response) = connect_async_tls_with_config(
        request,
        None,
        false,
        Some(Connector::NativeTls(
            native_tls::TlsConnector::builder().build()?,
        )),
    )
    .await?;
    println!("[ws] Connected. Sending connection_init...");

    // 1. Send connection_init with auth token
    let init_msg = serde_json::json!({
        "type": "connection_init",
        "payload": {
            "token": config.csgoroll_api_token
        }
    });
    ws.send(Message::Text(init_msg.to_string())).await?;

    // 2. Wait for connection_ack
    loop {
        match ws.next().await {
            Some(Ok(Message::Text(text))) => {
                let parsed: WsMessage = serde_json::from_str(&text)?;
                match parsed.msg_type.as_str() {
                    "connection_ack" => {
                        println!("[ws] connection_ack received. Subscribing...");
                        break;
                    }
                    "connection_error" => {
                        return Err(format!("connection_error: {:?}", parsed.payload).into());
                    }
                    other => {
                        println!("[ws] Unexpected message before ack: {}", other);
                    }
                }
            }
            Some(Ok(Message::Ping(data))) => {
                ws.send(Message::Pong(data)).await?;
            }
            Some(Ok(_)) => {}
            Some(Err(e)) => return Err(Box::new(e)),
            None => return Err("Connection closed before connection_ack".into()),
        }
    }

    // 3. Send subscribe message
    // The subscription query is the exact OnCreateTrades subscription.
    // userId is passed as null — the server sends all public trades without it.
    let subscribe_msg = serde_json::json!({
        "id": "1",
        "type": "subscribe",
        "payload": {
            "variables": {
                "userId": null
            },
            "extensions": {},
            "operationName": "OnCreateTrades",
            "query": "subscription OnCreateTrades($userId: ID, $marketName: String, $maxMarkupPercent: Float, $maxPrice: UnsignedFloat, $minPrice: UnsignedFloat, $maxAvgPaintWear: UnsignedFloat, $minAvgPaintWear: UnsignedFloat, $activeSince: SequelizeDate, $activeUntil: SequelizeDate, $trackingTypeGroup: TradeTrackingTypeGroup, $categoryIds: [ID!], $stickersAppliedCount: Int, $rarities: [String!]) {\n  createTrades(\n    userId: $userId\n    marketName: $marketName\n    maxMarkupPercent: $maxMarkupPercent\n    maxPrice: $maxPrice\n    minPrice: $minPrice\n    maxAvgPaintWear: $maxAvgPaintWear\n    minAvgPaintWear: $minAvgPaintWear\n    activeSince: $activeSince\n    activeUntil: $activeUntil\n    trackingTypeGroup: $trackingTypeGroup\n    categoryIds: $categoryIds\n    stickersAppliedCount: $stickersAppliedCount\n    rarities: $rarities\n  ) {\n    trades {\n      ...SimpleTrade\n      __typename\n    }\n    __typename\n  }\n}\n\nfragment SimpleTrade on Trade {\n  id\n  depositorLastActiveAt\n  markupPercent\n  totalValue\n  avgPaintWear\n  hasStickers\n  tradeItems {\n    ...SimpleTradeItem\n    __typename\n  }\n  trackingType\n  avgPaintWearRange {\n    min\n    max\n    __typename\n  }\n  canJoinAfter\n  depositor {\n    id\n    steamDisplayName\n    steamId\n    __typename\n  }\n  suspectedTraderCanJoinAfter\n  status\n  steamAppName\n  customValue\n  createdAt\n  updatedAt\n  canJoin\n  expiresAt\n  withdrawer {\n    id\n    steamId\n    steamDisplayName\n    steamLevel\n    avatar\n    steamRegistrationDate\n    name\n    __typename\n  }\n  joinedAt\n  cancelReason\n  withdrawerSteamTradeUrl\n  __typename\n}\n\nfragment SimpleTradeItem on TradeItem {\n  id\n  marketName\n  markupPercent\n  customValue\n  steamExternalAssetId\n  itemVariant {\n    ...SimpleTradeItemVariant\n    __typename\n  }\n  stickers {\n    ...SimpleTradeItemSticker\n    __typename\n  }\n  value\n  patternPercentage\n  __typename\n}\n\nfragment SimpleTradeItemVariant on ItemVariant {\n  brand\n  color\n  name\n  iconUrl\n  rarity\n  depositable\n  itemId\n  id\n  value\n  currency\n  externalId\n  __typename\n}\n\nfragment SimpleTradeItemSticker on TradeItemSticker {\n  imageUrl\n  id\n  name\n  color\n  wear\n  brand\n  __typename\n}"
        }
    });
    ws.send(Message::Text(subscribe_msg.to_string())).await?;
    println!("[ws] Subscribe sent. Waiting for trade events...");

    // 4. Receive loop
    loop {
        match ws.next().await {
            Some(Ok(Message::Text(text))) => {
                if let Ok(msg) = serde_json::from_str::<WsMessage>(&text) {
                    match msg.msg_type.as_str() {
                        "next" => {
                            if let Some(payload) = msg.payload {
                                if let Some(trades_arr) = payload
                                    .get("data")
                                    .and_then(|d| d.get("createTrades"))
                                    .and_then(|ct| ct.get("trades"))
                                    .and_then(|t| t.as_array())
                                {
                                    for trade_val in trades_arr {
                                        if let Some(trade) = parse_trade(trade_val) {
                                            on_trade(trade);
                                        }
                                    }
                                }
                            }
                        }
                        "ping" => {
                            // graphql-transport-ws protocol ping: respond with pong
                            let pong = serde_json::json!({"type": "pong"});
                            let _ = ws.send(Message::Text(pong.to_string())).await;
                        }
                        "error" => {
                            eprintln!("[ws] Subscription error: {:?}", msg.payload);
                        }
                        "complete" => {
                            println!("[ws] Subscription completed by server.");
                            return Ok(());
                        }
                        other => {
                            // Ignore unknown message types silently (e.g. ka heartbeats)
                            let _ = other;
                        }
                    }
                }
            }
            Some(Ok(Message::Ping(data))) => {
                let _ = ws.send(Message::Pong(data)).await;
            }
            Some(Ok(Message::Close(_))) => {
                println!("[ws] Server closed connection.");
                return Ok(());
            }
            Some(Ok(_)) => {}
            Some(Err(e)) => return Err(Box::new(e)),
            None => {
                println!("[ws] Connection stream ended.");
                return Ok(());
            }
        }
    }
}

fn parse_trade(val: &serde_json::Value) -> Option<Trade> {
    let id = val.get("id")?.as_str()?.to_string();
    let markup_percent = val.get("markupPercent")?.as_f64()?;
    let total_value = val.get("totalValue")?.as_f64()?;
    let can_join = val.get("canJoin")?.as_bool()?;
    let status = val.get("status")?.as_str()?.to_string();

    let items = val.get("tradeItems")?.as_array()?;
    let item = items.first()?;

    let market_name = item.get("marketName")?.as_str()?.to_string();
    let variant = item.get("itemVariant")?;
    let brand = variant.get("brand")?.as_str()?.to_string();
    let skin_name = variant.get("name")?.as_str()?.to_string();
    let wear = variant.get("color")?.as_str()?.to_string();

    let is_stattrak = market_name.starts_with("StatTrak\u{2122}");

    Some(Trade {
        id,
        markup_percent,
        total_value,
        can_join,
        status,
        market_name,
        brand,
        skin_name,
        wear,
        is_stattrak,
    })
}

/// Runs the WebSocket connection indefinitely, reconnecting with exponential backoff on any error.
///
/// Backoff schedule: 1s -> 2s -> 4s -> 8s -> 16s -> 32s -> 60s (capped).
/// Backoff resets to 1s after a connection attempt that successfully received at least one trade.
///
/// This function never returns under normal operation. It only returns if the caller's
/// on_trade closure panics (which it should not).
pub async fn run_with_reconnect(
    config: &Config,
    mut on_trade: impl FnMut(Trade),
) {
    const BACKOFF_CAP_SECS: u64 = 60;
    const BACKOFF_RESET_SECS: u64 = 1;
    let mut delay_secs: u64 = BACKOFF_RESET_SECS;

    loop {
        // Wrap on_trade to track whether we received at least one trade this attempt
        let mut received_count: u32 = 0;
        let result = connect_once(config, |trade| {
            received_count += 1;
            on_trade(trade);
        })
        .await;

        match result {
            Ok(()) => {
                // Clean close — server closed the connection
                println!("[ws] Connection closed cleanly. Reconnecting in {}s...", delay_secs);
            }
            Err(e) => {
                eprintln!("[ws] Connection error: {}. Reconnecting in {}s...", e, delay_secs);
            }
        }

        // Reset backoff if this connection was productive
        if received_count > 0 {
            delay_secs = BACKOFF_RESET_SECS;
            println!("[ws] Productive session ({} trades). Backoff reset to 1s.", received_count);
        }

        tokio::time::sleep(Duration::from_secs(delay_secs)).await;

        // Advance backoff for next failure (only advances if we didn't reset)
        if received_count == 0 {
            delay_secs = (delay_secs * 2).min(BACKOFF_CAP_SECS);
        }
    }
}
