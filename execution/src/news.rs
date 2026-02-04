use feed_rs::parser;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct NewsItem {
    pub source: String,
    pub title: String,
    pub url: String,
    pub ts_unix_ms: i64,
}

fn now_ms() -> i64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as i64
}

async fn fetch_feed(url: &str) -> anyhow::Result<Vec<NewsItem>> {
    let bytes = reqwest::Client::new()
        .get(url)
        .header("User-Agent", "BaconAlgo/1.0")
        .send()
        .await?
        .bytes()
        .await?;

    let feed = parser::parse(&bytes[..])?;

    let source = feed
        .title
        .as_ref()
        .map(|t| t.content.clone())
        .unwrap_or_else(|| "Feed".to_string());

    let mut items = Vec::new();
    for entry in feed.entries.into_iter().take(25) {
        let title = entry
            .title
            .as_ref()
            .map(|t| t.content.clone())
            .unwrap_or_else(|| "Untitled".to_string());

        let url = entry
            .links
            .get(0)
            .map(|l| l.href.clone())
            .unwrap_or_else(|| "".to_string());

        let ts = entry
            .published
            .or(entry.updated)
            .map(|d| d.timestamp_millis())
            .unwrap_or_else(now_ms);

        items.push(NewsItem {
            source: source.clone(),
            title,
            url,
            ts_unix_ms: ts,
        });
    }

    Ok(items)
}

pub async fn get_live_news() -> Vec<NewsItem> {
    // Mix “macro/markets + crypto” sans API keys (RSS).
    // Tu peux ajuster la liste plus tard.
    let feeds = [
        // Markets / Macro
        "https://www.investing.com/rss/news_25.rss",
        "https://www.cnbc.com/id/100003114/device/rss/rss.html",
        // Crypto
        "https://www.coindesk.com/arc/outboundfeeds/rss/",
        "https://cointelegraph.com/rss",
    ];

    let mut all = Vec::new();
    for f in feeds {
        if let Ok(mut items) = fetch_feed(f).await {
            all.append(&mut items);
        }
    }

    // Tri + dédoublonnage simple par (title)
    all.sort_by(|a, b| b.ts_unix_ms.cmp(&a.ts_unix_ms));

    let mut seen = std::collections::HashSet::new();
    all.into_iter()
        .filter(|x| seen.insert(x.title.clone()))
        .take(25)
        .collect()
}
