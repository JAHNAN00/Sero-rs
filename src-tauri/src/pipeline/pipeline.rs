use crate::core::types::PipelineItem;
use crate::pipeline::stages::Stage;

pub struct Pipeline {
    id: String,
    stages: Vec<Box<dyn Stage + Send + Sync>>,
}

impl Pipeline {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            stages: Vec::new(),
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn push_stage(&mut self, stage: Box<dyn Stage + Send + Sync>) {
        self.stages.push(stage);
    }

    pub fn process(&self, input: PipelineItem) -> Vec<PipelineItem> {
        let mut items = vec![input];
        for stage in &self.stages {
            let mut next = Vec::new();
            for item in items {
                next.extend(stage.process(item));
            }
            items = next;
        }
        items
    }
}
