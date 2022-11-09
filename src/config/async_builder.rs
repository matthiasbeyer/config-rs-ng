pub struct AsyncConfigBuilder {
    layers_builders: Vec<Box<dyn crate::source::AsyncConfigSource>>,
    defaults_builders: Vec<Box<dyn crate::source::AsyncConfigSource>>,
    overwrites_builders: Vec<Box<dyn crate::source::AsyncConfigSource>>,
}

impl AsyncConfigBuilder {
    pub(crate) fn new() -> Self {
        Self {
            layers_builders: Vec::new(),
            defaults_builders: Vec::new(),
            overwrites_builders: Vec::new(),
        }
    }
    pub fn load_async(mut self, source: Box<dyn crate::source::AsyncConfigSource>) -> Self {
        self.layers_builders.push(source);
        self
    }
}
