use color_eyre::eyre::Result;
use std::time::Duration;
use tracing_error::ErrorLayer;
use tracing_subscriber::prelude::*;
use tracing_subscriber::{fmt, EnvFilter};

use axum::{body::Body, extract::Request, response::Response};
use tracing::{Level, Span};

pub fn init_tracing() -> Result<()> {
    // Create a formatting layer for tracing output with a compact format
    let fmt_layer = fmt::layer().compact();

    // Create a filter layer to control the verbosity of tracing output
    let filter_layer = EnvFilter::try_from_default_env().or_else(|_| EnvFilter::try_new("info"))?;

    // Build the tracing subscriber registry with the formatting layer,
    // the filter layer, and the error layer
    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .with(ErrorLayer::default())
        .init();

    Ok(())

    // tracing_subscriber::fmt()
    //     .compact()
    //     .with_max_level(tracing::Level::DEBUG)
    //     .init();
}

pub fn make_span_with_request_id(request: &Request<Body>) -> Span {
    let request_id = uuid::Uuid::new_v4();
    tracing::span!(
        Level::INFO,
        "[REQUEST]",
        method = tracing::field::display(request.method()),
        uri = tracing::field::display(request.uri()),
        // The version field is removed because `axum::http::Version` does not implement `std::fmt::Display`
        // version = tracing::field::display(request.version()),
        request_id = tracing::field::display(request_id),
    )
}

pub fn on_request(_request: &Request<Body>, _span: &Span) {
    tracing::event!(Level::INFO, "[REQUEST START]");
}

pub fn on_response(response: &Response, latency: Duration, _span: &Span) {
    let status = response.status();
    let status_code = status.as_u16();
    let status_code_class = status_code / 100;

    match status_code_class {
        4..=5 => {
            tracing::event!(
                Level::ERROR,
                latency = ?latency,
                status = status_code,
                "[REQUEST END]"
            )
        }
        _ => {
            tracing::event!(
                Level::INFO,
                latency = ?latency,
                status = status_code,
                "[REQUEST END]"
            )
        }
    };
}
