/**
 * SRS Exporter
 * Fetch SRS Status by http request, integrate with prometheus client.
 */
mod collector;
pub use collector::{Collector, StreamUsage};

/**
 * Deal with http requests
 */
mod server;

struct SrsExporterConfig {
    srs_host: String,
    srs_port: u32,
}
