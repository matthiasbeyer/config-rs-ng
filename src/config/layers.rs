use crate::accessor::Accessor;
use crate::config::ConfigError;
use crate::element::ConfigElement;
use crate::object::ConfigObject;
use crate::object::ConfigView;
use crate::ParsableAccessor;

#[derive(Debug)]
pub struct Layers(Vec<ConfigObject>);

impl FromIterator<ConfigObject> for Layers {
    fn from_iter<T: IntoIterator<Item = ConfigObject>>(iter: T) -> Self {
        Layers(Vec::from_iter(iter))
    }
}

impl Layers {
    /// Access the configuration at a specific position
    ///
    /// Use an object of a type implementing the `ParsableAccessor` trait for accessing the
    /// configuration at a certain position.
    /// As `ParsableAccessor` is implemented by [`&str`] and [`String`], passing those directly
    /// works.
    ///
    /// # Note
    ///
    /// Each time, [`Config::get`] is called, the `ParsableAccessor::parse()` function is called.
    /// If that is a unbearable overhead (especially in cases where the accessor is hard-coded),
    /// [`Config::get_with_accessor`] can be used to prevent that overhead.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use config_rs_ng::Config;
    /// let config: Config = { //...
    /// # unimplemented!()
    /// };
    ///
    /// config.layers().get("foo")
    ///     // ...
    /// # ;
    /// ```
    pub fn get<A>(&self, accessor: A) -> Result<Option<&dyn ConfigElement>, ConfigError>
    where
        A: ParsableAccessor,
    {
        let accessor = accessor.parse()?;
        self.get_with_accessor(accessor)
    }

    /// Get a "View" from the configuration
    ///
    /// The function works like [`Config::get`], except that it wraps the `&dyn ConfigElement` in a
    /// `ConfigView` that also contains the description of the source of the configuration value.
    pub fn get_view<A>(&self, accessor: A) -> Result<Option<ConfigView<'_>>, ConfigError>
    where
        A: ParsableAccessor,
    {
        let accessor = accessor.parse()?;
        self.get_view_with_accessor(accessor)
    }

    pub fn get_as<A, T>(&self, accessor: A) -> Result<Option<&T>, ConfigError>
    where
        A: ParsableAccessor,
        T: ConfigElement,
    {
        let accessor = accessor.parse()?;
        self.get_with_accessor_as::<T>(accessor)
    }

    /// Access the configuration at a specific position
    ///
    /// See [`Layers::get`]
    pub fn get_with_accessor(
        &self,
        mut accessor: Accessor,
    ) -> Result<Option<&dyn ConfigElement>, ConfigError> {
        for layer in self.0.iter().rev() {
            if let Some(value) = layer.get(&mut accessor)? {
                return Ok(Some(value));
            }
        }

        Ok(None)
    }

    /// Access the configuration at a specific position, and return the description of the value as
    /// well
    ///
    /// See [`Layers::get_view`]
    pub fn get_view_with_accessor(
        &self,
        mut accessor: Accessor,
    ) -> Result<Option<ConfigView<'_>>, ConfigError> {
        for layer in self.0.iter().rev() {
            if let Some(view) = layer.get_with_description(&mut accessor)? {
                return Ok(Some(view));
            }
        }

        Ok(None)
    }

    pub fn get_with_accessor_as<T>(&self, mut accessor: Accessor) -> Result<Option<&T>, ConfigError>
    where
        T: ConfigElement,
    {
        for layer in self.0.iter().rev() {
            if let Some(value) = layer.get_as::<T>(&mut accessor)? {
                return Ok(Some(value));
            }
        }

        Ok(None)
    }
}
