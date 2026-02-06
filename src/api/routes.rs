use axum::{
    extract::State,
    response::sse::{Event, Sse},
    routing::{get, post, put},
    Router,
};
use tokio_stream::{wrappers::BroadcastStream, StreamExt};
use std::convert::Infallible;

use crate::bus::SignalBus;
use crate::api::models::LiveSignal;

mod historical;
mod portfolio;
mod preferences;
mod websocket;

pub async fn sse_signals(
    State(bus): State<SignalBus<LiveSignal>>,
) -> Sse<impl tokio_stream::Stream<Item = Result<Event, Infallible>>> {
    let rx = bus.subscribe();
let stream = BroadcastStream::new(rx)
    .map(|msg| {
        match msg {
            Ok(signal) => {
                serde_json::to_string(&signal)
                    .ok()
                    .map(|json| Ok(Event::default().data(json)))
            }
            Err(_) => None,
        }
    })
    .filter_map(|x| x);

    Sse::new(stream)
}

pub fn api_routes() -> Router {
    Router::new()
        .route("/api/historical/:symbol", get(historical::get_historical))
        .route("/api/portfolio", get(portfolio::get_portfolio))
        .route("/api/portfolio/position", post(portfolio::create_order))
        .route("/api/preferences", get(preferences::get_preferences))
        .route("/api/preferences", put(preferences::update_preferences))
        .route("/ws", get(websocket::websocket_handler))
}
