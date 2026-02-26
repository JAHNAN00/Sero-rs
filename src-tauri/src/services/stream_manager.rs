use std::collections::HashMap;

use crate::core::traits::DataSource;
use crate::core::types::{
    DataPacket, ParserDescriptor, PipelineItem, SourceInfo, SourceKind, SourceStatus,
};
use crate::pipeline::stages::{FloatExtractor, LineSplitter, Stage};
use crate::pipeline::Pipeline;
use crate::services::registry::ParserRegistry;

pub struct StreamManager {
    sources: HashMap<String, Box<dyn DataSource>>,
    pipelines: HashMap<String, Pipeline>,
    source_pipelines: HashMap<String, String>,
    registry: ParserRegistry,
}

impl StreamManager {
    pub fn new() -> Self {
        Self {
            sources: HashMap::new(),
            pipelines: HashMap::new(),
            source_pipelines: HashMap::new(),
            registry: ParserRegistry::new(),
        }
    }

    pub fn list_sources(&self) -> Vec<SourceInfo> {
        self.sources
            .values()
            .map(|source| SourceInfo {
                id: source.id().to_string(),
                label: source.label().to_string(),
                kind: source.kind(),
                status: source.status(),
            })
            .collect()
    }

    pub fn list_parsers(&self) -> Vec<ParserDescriptor> {
        self.registry.list()
    }

    pub fn add_source(&mut self, source: Box<dyn DataSource>) {
        let id = source.id().to_string();
        self.sources.insert(id, source);
    }

    pub fn start_source(&mut self, source_id: &str) -> Result<(), String> {
        let source = self
            .sources
            .get_mut(source_id)
            .ok_or_else(|| format!("source not found: {source_id}"))?;
        source.start()
    }

    pub fn stop_source(&mut self, source_id: &str) -> Result<(), String> {
        let source = self
            .sources
            .get_mut(source_id)
            .ok_or_else(|| format!("source not found: {source_id}"))?;
        source.stop()
    }

    pub fn attach_pipeline(&mut self, source_id: &str, pipeline: Pipeline) -> Result<(), String> {
        if !self.sources.contains_key(source_id) {
            return Err(format!("source not found: {source_id}"));
        }

        // TODO: Wire the pipeline into the source's output stream.
        let pipeline_id = pipeline.id().to_string();
        self.pipelines.insert(pipeline_id.clone(), pipeline);
        self.source_pipelines
            .insert(source_id.to_string(), pipeline_id);
        Ok(())
    }

    pub fn pipeline_status(&self, pipeline_id: &str) -> Option<(String, SourceKind, SourceStatus)> {
        self.pipelines.get(pipeline_id).map(|pipeline| {
            (
                pipeline.id().to_string(),
                SourceKind::Unknown,
                SourceStatus::Stopped,
            )
        })
    }

    pub fn ingest_packet(
        &mut self,
        source_id: &str,
        packet: DataPacket,
    ) -> Result<(String, Vec<PipelineItem>), String> {
        let pipeline_id = self.ensure_demo_pipeline(source_id)?;
        let pipeline = self
            .pipelines
            .get(&pipeline_id)
            .ok_or_else(|| format!("pipeline not found: {pipeline_id}"))?;
        Ok((pipeline_id, pipeline.process(PipelineItem::Packet(packet))))
    }

    fn ensure_demo_pipeline(&mut self, source_id: &str) -> Result<String, String> {
        if !self.sources.contains_key(source_id) {
            return Err(format!("source not found: {source_id}"));
        }

        if let Some(pipeline_id) = self.source_pipelines.get(source_id) {
            return Ok(pipeline_id.clone());
        }

        let pipeline_id = format!("{source_id}_demo");
        let mut pipeline = Pipeline::new(&pipeline_id);
        pipeline.push_stage(Box::new(LineSplitter) as Box<dyn Stage + Send + Sync>);
        pipeline.push_stage(Box::new(FloatExtractor) as Box<dyn Stage + Send + Sync>);
        self.pipelines.insert(pipeline_id.clone(), pipeline);
        self.source_pipelines
            .insert(source_id.to_string(), pipeline_id.clone());
        Ok(pipeline_id)
    }
}
