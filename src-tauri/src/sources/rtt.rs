use std::sync::mpsc::Sender;

use crate::core::traits::DataSource;
use crate::core::types::{DataPacket, SourceKind, SourceStatus};

pub struct RttSource {
    id: String,
    label: String,
    status: SourceStatus,
    #[allow(dead_code)]
    sender: Option<Sender<DataPacket>>,
}

impl RttSource {
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            status: SourceStatus::Stopped,
            sender: None,
        }
    }
}

impl DataSource for RttSource {
    fn id(&self) -> &str {
        &self.id
    }

    fn label(&self) -> &str {
        &self.label
    }

    fn kind(&self) -> SourceKind {
        SourceKind::Rtt
    }

    fn status(&self) -> SourceStatus {
        self.status.clone()
    }

    fn start(&mut self) -> Result<(), String> {
        // TODO: Initialize RTT session and start pushing DataPacket to sender.
        self.status = SourceStatus::Running;
        Ok(())
    }

    fn stop(&mut self) -> Result<(), String> {
        // TODO: Stop RTT session and release hardware resources.
        self.status = SourceStatus::Stopped;
        Ok(())
    }

    fn set_sender(&mut self, sender: Sender<DataPacket>) {
        self.sender = Some(sender);
    }
}
