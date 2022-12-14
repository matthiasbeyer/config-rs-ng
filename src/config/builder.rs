use crate::config::Config;
use crate::source::ConfigSource;
use crate::source::SourceError;

use super::ConfigError;
use crate::config::layers::Layers;

#[derive(Debug)]
pub struct ConfigBuilder {
    layers_builders: Vec<Box<dyn ConfigSource>>,
    defaults_builders: Vec<Box<dyn ConfigSource>>,
    overwrites_builders: Vec<Box<dyn ConfigSource>>,
}

impl ConfigBuilder {
    pub(crate) fn new() -> Self {
        ConfigBuilder {
            layers_builders: Vec::new(),
            defaults_builders: Vec::new(),
            overwrites_builders: Vec::new(),
        }
    }

    pub fn load(mut self, source: Box<dyn ConfigSource>) -> Self {
        self.layers_builders.push(source);
        self
    }

    pub fn load_default(mut self, source: Box<dyn ConfigSource>) -> Self {
        self.defaults_builders.push(source);
        self
    }

    pub fn load_overwrite(mut self, source: Box<dyn ConfigSource>) -> Self {
        self.overwrites_builders.push(source);
        self
    }

    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub fn build(self) -> Result<Config, ConfigError> {
        Config::build_from_builder(self)
    }

    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub(crate) fn reload(&self) -> Result<Layers, SourceError> {
        self.overwrites_builders
            .iter()
            .map(|cs| cs.load())
            .chain(self.layers_builders.iter().map(|cs| cs.load()))
            .chain(self.defaults_builders.iter().map(|cs| cs.load()))
            .collect()
    }
}
