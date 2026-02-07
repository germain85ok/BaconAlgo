mod bus;
mod api;
mod scanner;
mod market;
mod news;
mod config;
mod families;
mod engine;
mod backtest;
mod smc;
mod signal;

use axum::{
    routing::{get, post},
    Router,
    http::{header, Method},
};
use std::net::SocketAddr;
use std::sync::Arc;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use api::{
    routes::sse_signals, 
    signals::{get_signals, stream_signals},
    market::{get_fear_greed_index, get_vix, get_movers},
    news::get_news,
    get_performance_metrics,
    LiveSignal,
};
use bus::SignalBus;
use config::CONFIG;
use scanner::Scanner;
use market::ProviderManager;

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "execution=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("🚀 BaconAlgo Execution Engine starting...");

    // Validate configuration
    if let Err(e) = CONFIG.validate() {
        tracing::error!("Configuration validation failed: {}", e);
        tracing::warn!("Continuing with partial configuration for development...");
    }

    // Create signal bus with capacity for 256 messages
    let bus = SignalBus::<LiveSignal>::new(256);

    // Create provider manager for real market data
    let provider_manager = Arc::new(ProviderManager::new());
    tracing::info!("✅ Provider manager initialized");

    // Start scanner in background
    let scanner_bus = bus.clone();
    let scanner_provider = provider_manager.clone();
    tokio::spawn(async move {
        let mut scanner = Scanner::new(scanner_bus.sender(), scanner_provider);
        
        // Add real indicators to scanner
        // For now, we run without indicators - they can be added later
        // scanner.add_indicator(Arc::new(SomeIndicator::new()));
        
        scanner.run().await;
    });

    // Setup CORS
    let cors = CorsLayer::new()
        .allow_origin(
            CONFIG.cors_origins
                .iter()
                .map(|origin| origin.parse().unwrap())
                .collect::<Vec<_>>()
        )
        .allow_methods([Method::GET, Method::POST, Method::OPTIONS])
        .allow_headers([header::CONTENT_TYPE, header::AUTHORIZATION]);

    // Build router
    let app = Router::new()
        // Health check
        .route("/health", get(health_check))
        
        // Signal endpoints
        .route("/api/signals", get(get_signals))
        .route("/api/signals/stream", get(stream_signals))
        
        // Legacy SSE endpoint (keep for backwards compatibility)
        .route("/signals/live", get(sse_signals))
        
        // Market data endpoints
        .route("/api/market/fear-greed", get(get_fear_greed_index))
        .route("/api/market/vix", get(get_vix))
        .route("/api/market/movers", get(get_movers))
        
        // News endpoint
        .route("/api/news", get(get_news))
        
        // Performance metrics endpoint
        .route("/api/metrics", get(get_performance_metrics))
        
        // Manual scan trigger
        .route("/api/scan", post(trigger_scan))
        
        .with_state(bus)
        .layer(
            ServiceBuilder::new()
                .layer(TraceLayer::new_for_http())
                .layer(cors)
        );

    // Start server
    let addr = SocketAddr::from(([0, 0, 0, 0], CONFIG.server_port));
    tracing::info!("🔥 Server listening on http://{}", addr);
    tracing::info!("📡 SSE endpoint: http://{}/api/signals/stream", addr);
    tracing::info!("💓 Health check: http://{}/health", addr);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind to address");

    axum::serve(listener, app)
        .await
        .expect("Server error");
}

/// Health check endpoint
async fn health_check() -> axum::Json<serde_json::Value> {
    axum::Json(serde_json::json!({
        "status": "ok",
        "service": "BaconAlgo Execution Engine",
        "version": env!("CARGO_PKG_VERSION"),
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "config": {
            "has_market_provider": CONFIG.has_market_provider(),
            "has_crypto_apis": CONFIG.has_crypto_apis(),
            "has_broker_apis": CONFIG.has_broker_apis(),
            "has_database": CONFIG.has_database(),
        }
    }))
}

/// Manual scan trigger endpoint
async fn trigger_scan() -> axum::Json<serde_json::Value> {
    tracing::info!("Manual scan triggered via API");
    
    // TODO: Implement manual scan trigger
    axum::Json(serde_json::json!({
        "status": "triggered",
        "message": "Scan initiated",
        "timestamp": chrono::Utc::now().to_rfc3339(),
    }))
}

