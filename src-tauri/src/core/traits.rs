use std::sync::mpsc::Sender;

use crate::core::types::{DataPacket, SourceKind, SourceStatus};

pub trait DataSource: Send {
    fn id(&self) -> &str;
    fn label(&self) -> &str;
    fn kind(&self) -> SourceKind;
    fn status(&self) -> SourceStatus;
    fn start(&mut self) -> Result<(), String>;
    fn stop(&mut self) -> Result<(), String>;

    // TODO: Replace with async channel and backpressure handling.
    fn set_sender(&mut self, sender: Sender<DataPacket>);
}
