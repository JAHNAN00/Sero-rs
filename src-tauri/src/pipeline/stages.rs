use crate::core::types::{now_millis, DataPacket, Metric, PipelineItem};

#[allow(dead_code)]
pub trait Stage {
    fn name(&self) -> &str;
    fn process(&self, item: PipelineItem) -> Vec<PipelineItem>;
}

pub struct LineSplitter;

impl Stage for LineSplitter {
    fn name(&self) -> &str {
        "line_splitter"
    }

    fn process(&self, item: PipelineItem) -> Vec<PipelineItem> {
        match item {
            PipelineItem::Packet(packet) => split_lines(packet)
                .into_iter()
                .map(PipelineItem::Packet)
                .collect(),
            other => vec![other],
        }
    }
}

pub struct FloatExtractor;

impl Stage for FloatExtractor {
    fn name(&self) -> &str {
        "float_extractor"
    }

    fn process(&self, item: PipelineItem) -> Vec<PipelineItem> {
        match item {
            PipelineItem::Packet(packet) => extract_floats(packet)
                .into_iter()
                .map(PipelineItem::Metric)
                .collect(),
            other => vec![other],
        }
    }
}

fn split_lines(packet: DataPacket) -> Vec<DataPacket> {
    let text = match packet.text.clone() {
        Some(text) => text,
        None => return vec![packet],
    };

    let mut out = Vec::new();
    for line in text.lines() {
        if line.is_empty() {
            continue;
        }
        out.push(DataPacket {
            ts_millis: packet.ts_millis,
            source_id: packet.source_id.clone(),
            raw: line.as_bytes().to_vec(),
            text: Some(line.to_string()),
            tags: packet.tags.clone(),
        });
    }
    if out.is_empty() {
        vec![packet]
    } else {
        out
    }
}

fn extract_floats(packet: DataPacket) -> Vec<Metric> {
    let text = match packet.text {
        Some(text) => text,
        None => return Vec::new(),
    };

    // TODO: Replace with a robust parser that can handle scientific notation,
    // signed numbers, and mixed tokens without splitting.
    let sanitized = text
        .replace(',', " ")
        .replace(';', " ")
        .replace('\t', " ")
        .replace('|', " ");

    let mut metrics = Vec::new();
    for token in sanitized.split_whitespace() {
        if let Ok(value) = token.parse::<f64>() {
            metrics.push(Metric {
                ts_millis: now_millis(),
                source_id: packet.source_id.clone(),
                name: "float".to_string(),
                value,
            });
        }
    }
    metrics
}
