use axum::{
    extract::State,
    response::sse::{Event, Sse},
};
use tokio_stream::{wrappers::BroadcastStream, StreamExt};
use std::convert::Infallible;

use crate::bus::SignalBus;
use crate::api::models::LiveSignal;

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
