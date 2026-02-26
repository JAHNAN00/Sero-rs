use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataPacket {
    pub ts_millis: u128,
    pub source_id: String,
    pub raw: Vec<u8>,
    pub text: Option<String>,
    pub tags: Vec<String>,
}

impl DataPacket {
    pub fn new(source_id: impl Into<String>, raw: Vec<u8>, text: Option<String>) -> Self {
        Self {
            ts_millis: now_millis(),
            source_id: source_id.into(),
            raw,
            text,
            tags: Vec::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedEvent {
    pub ts_millis: u128,
    pub kind: String,
    pub payload: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metric {
    pub ts_millis: u128,
    pub source_id: String,
    pub name: String,
    pub value: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PipelineItem {
    Packet(DataPacket),
    Event(ParsedEvent),
    Metric(Metric),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SourceKind {
    Rtt,
    Serial,
    Network,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SourceStatus {
    Stopped,
    Running,
    Error(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceInfo {
    pub id: String,
    pub label: String,
    pub kind: SourceKind,
    pub status: SourceStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParserDescriptor {
    pub id: String,
    pub label: String,
    pub kind: String,
    pub configurable: bool,
}

pub fn now_millis() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
}
