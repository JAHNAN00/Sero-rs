use tauri::{AppHandle, Emitter};

use crate::core::types::{Metric, ParsedEvent};

#[allow(dead_code)]
pub struct SubscriptionHub {
    app: AppHandle,
}

#[allow(dead_code)]
impl SubscriptionHub {
    pub fn new(app: AppHandle) -> Self {
        Self { app }
    }

    pub fn emit_metric(&self, event: &str, metric: &Metric) -> Result<(), String> {
        self.app
            .emit(event, metric)
            .map_err(|err| err.to_string())
    }

    pub fn emit_event(&self, event: &str, parsed: &ParsedEvent) -> Result<(), String> {
        self.app
            .emit(event, parsed)
            .map_err(|err| err.to_string())
    }

    // TODO: Track active subscriptions and allow per-source filtering.
}
