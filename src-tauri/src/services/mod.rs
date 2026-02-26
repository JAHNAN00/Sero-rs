pub mod registry;
pub mod stream_manager;
pub mod subscription;

use std::sync::{Arc, Mutex};

#[allow(unused_imports)]
pub use registry::ParserRegistry;
pub use stream_manager::StreamManager;
#[allow(unused_imports)]
pub use subscription::SubscriptionHub;

use crate::sources::{NetworkSource, RttSource, SerialSource};

pub struct AppState {
    pub manager: Arc<Mutex<StreamManager>>,
}

impl AppState {
    pub fn new() -> Self {
        let mut manager = StreamManager::new();
        manager.add_source(Box::new(SerialSource::new("serial", "Serial")));
        manager.add_source(Box::new(RttSource::new("rtt", "RTT")));
        manager.add_source(Box::new(NetworkSource::new("network", "Network")));

        Self {
            manager: Arc::new(Mutex::new(manager)),
        }
    }
}
