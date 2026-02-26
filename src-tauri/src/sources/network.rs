use std::sync::mpsc::Sender;

use crate::core::traits::DataSource;
use crate::core::types::{DataPacket, SourceKind, SourceStatus};

pub struct NetworkSource {
    id: String,
    label: String,
    status: SourceStatus,
    sender: Option<Sender<DataPacket>>,
}

impl NetworkSource {
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            status: SourceStatus::Stopped,
            sender: None,
        }
    }
}

impl DataSource for NetworkSource {
    fn id(&self) -> &str {
        &self.id
    }

    fn label(&self) -> &str {
        &self.label
    }

    fn kind(&self) -> SourceKind {
        SourceKind::Network
    }

    fn status(&self) -> SourceStatus {
        self.status.clone()
    }

    fn start(&mut self) -> Result<(), String> {
        // TODO: Bind socket, start receive loop, emit DataPacket to sender.
        self.status = SourceStatus::Running;
        Ok(())
    }

    fn stop(&mut self) -> Result<(), String> {
        // TODO: Close socket and cleanup.
        self.status = SourceStatus::Stopped;
        Ok(())
    }

    fn set_sender(&mut self, sender: Sender<DataPacket>) {
        self.sender = Some(sender);
    }
}
