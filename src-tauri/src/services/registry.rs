use crate::core::types::ParserDescriptor;

pub struct ParserRegistry {
    parsers: Vec<ParserDescriptor>,
}

impl ParserRegistry {
    pub fn new() -> Self {
        let parsers = vec![
            ParserDescriptor {
                id: "line_splitter".to_string(),
                label: "Line Splitter".to_string(),
                kind: "stage".to_string(),
                configurable: false,
            },
            ParserDescriptor {
                id: "float_extractor".to_string(),
                label: "Float Extractor".to_string(),
                kind: "stage".to_string(),
                configurable: false,
            },
        ];

        Self { parsers }
    }

    pub fn list(&self) -> Vec<ParserDescriptor> {
        self.parsers.clone()
    }
}
