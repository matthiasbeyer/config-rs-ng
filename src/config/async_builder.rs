use futures::stream::FuturesUnordered;

use crate::{
    source::{AsyncConfigSource, SourceError},
    AsyncConfig, ConfigObject,
};

use super::ConfigError;

#[derive(Debug)]
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

    pub fn load(mut self, source: Box<dyn AsyncConfigSource>) -> Self {
        self.layers_builders.push(source);
        self
    }

    pub fn load_default(mut self, source: Box<dyn AsyncConfigSource>) -> Self {
        self.defaults_builders.push(source);
        self
    }

    pub fn load_overwrite(mut self, source: Box<dyn AsyncConfigSource>) -> Self {
        self.overwrites_builders.push(source);
        self
    }

    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub async fn build(self) -> Result<AsyncConfig, ConfigError> {
        AsyncConfig::build_from_builder(self).await
    }

    #[cfg_attr(feature = "tracing", tracing::instrument)]
    pub(crate) async fn reload(&self) -> Result<Vec<ConfigObject>, SourceError> {
        use futures::StreamExt;

        let overrides = self
            .overwrites_builders
            .iter()
            .map(|cs| cs.load_async())
            .collect::<FuturesUnordered<_>>()
            .collect::<Vec<_>>();

        let layers = self
            .layers_builders
            .iter()
            .map(|cs| cs.load_async())
            .collect::<FuturesUnordered<_>>()
            .collect::<Vec<_>>();

        let defaults = self
            .defaults_builders
            .iter()
            .map(|cs| cs.load_async())
            .collect::<FuturesUnordered<_>>()
            .collect::<Vec<_>>();

        let (overrides, layers, defaults) = {
            #[cfg(feature = "tracing")]
            {
                use tracing::Instrument;
                (
                    overrides.instrument(tracing::debug_span!("Loading overrides")),
                    layers.instrument(tracing::debug_span!("Loading layers")),
                    defaults.instrument(tracing::debug_span!("Loading defaults")),
                )
            }
            #[cfg(not(feature = "tracing"))]
            {
                (overrides, layers, defaults)
            }
        };

        let (overrides, layers, defaults) = futures::join!(overrides, layers, defaults);

        overrides
            .into_iter()
            .chain(layers.into_iter())
            .chain(defaults.into_iter())
            .collect()
    }
}
