use tauri::{AppHandle, Emitter, State};

use crate::api::events::{data_stream_event, metrics_event};
use crate::core::types::{DataPacket, ParserDescriptor, PipelineItem, SourceInfo};
use crate::services::AppState;

#[tauri::command]
pub fn list_sources(state: State<AppState>) -> Result<Vec<SourceInfo>, String> {
    let manager = state.manager.lock().map_err(|_| "lock poisoned".to_string())?;
    Ok(manager.list_sources())
}

#[tauri::command]
pub fn list_parsers(state: State<AppState>) -> Result<Vec<ParserDescriptor>, String> {
    let manager = state.manager.lock().map_err(|_| "lock poisoned".to_string())?;
    Ok(manager.list_parsers())
}

#[tauri::command]
pub fn start_source(state: State<AppState>, source_id: String) -> Result<(), String> {
    let mut manager = state.manager.lock().map_err(|_| "lock poisoned".to_string())?;
    manager.start_source(&source_id)
}

#[tauri::command]
pub fn stop_source(state: State<AppState>, source_id: String) -> Result<(), String> {
    let mut manager = state.manager.lock().map_err(|_| "lock poisoned".to_string())?;
    manager.stop_source(&source_id)
}

#[tauri::command]
pub fn attach_pipeline(
    state: State<AppState>,
    source_id: String,
    pipeline_id: String,
) -> Result<(), String> {
    let mut manager = state.manager.lock().map_err(|_| "lock poisoned".to_string())?;
    // TODO: Create pipeline from configuration and attach to source.
    let pipeline = crate::pipeline::Pipeline::new(pipeline_id);
    manager.attach_pipeline(&source_id, pipeline)
}

#[tauri::command]
pub fn mock_rx(app: AppHandle, state: State<AppState>, source_id: String, text: String) -> Result<(), String> {
    let mut manager = state.manager.lock().map_err(|_| "lock poisoned".to_string())?;
    let packet = DataPacket::new(&source_id, text.as_bytes().to_vec(), Some(text));

    // 先发送原始数据包，再走管线输出解析结果
    app.emit(&data_stream_event(&source_id), &packet)
        .map_err(|err| err.to_string())?;

    let (pipeline_id, outputs) = manager.ingest_packet(&source_id, packet)?;
    let metrics_event = metrics_event(&pipeline_id);

    for item in outputs {
        match item {
            PipelineItem::Metric(metric) => {
                app.emit(&metrics_event, &metric)
                    .map_err(|err| err.to_string())?;
            }
            PipelineItem::Event(event) => {
                app.emit(&data_stream_event(&source_id), &event)
                    .map_err(|err| err.to_string())?;
            }
            PipelineItem::Packet(packet) => {
                app.emit(&data_stream_event(&source_id), &packet)
                    .map_err(|err| err.to_string())?;
            }
        }
    }

    Ok(())
}
