pub const DATA_STREAM_PREFIX: &str = "data_stream";
pub const METRICS_PREFIX: &str = "metrics";

pub fn data_stream_event(source_id: &str) -> String {
    format!("{DATA_STREAM_PREFIX}::{source_id}")
}

pub fn metrics_event(pipeline_id: &str) -> String {
    format!("{METRICS_PREFIX}::{pipeline_id}")
}
