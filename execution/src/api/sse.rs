use axum::response::sse::Event;
use serde::Serialize;
use std::convert::Infallible;
use tokio_stream::Stream;
use std::pin::Pin;

/// Helper type for SSE streams
pub type SseStream = Pin<Box<dyn Stream<Item = Result<Event, Infallible>> + Send>>;

/// Create an SSE event from any serializable data
pub fn create_event<T: Serialize>(data: &T) -> Result<Event, serde_json::Error> {
    let json = serde_json::to_string(data)?;
    Ok(Event::default().data(json))
}

/// Create a named SSE event
pub fn create_named_event<T: Serialize>(
    event_name: &str,
    data: &T,
) -> Result<Event, serde_json::Error> {
    let json = serde_json::to_string(data)?;
    Ok(Event::default().event(event_name).data(json))
}

/// Create a heartbeat event to keep connection alive
pub fn heartbeat_event() -> Event {
    Event::default().comment("heartbeat")
}
