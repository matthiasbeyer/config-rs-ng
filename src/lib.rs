//! # config_rs_ng
//!
//! `config_rs_ng` organizes hierarchical or layered configurations for Rust applications.
//!
//! It lets you merge configuration from a variety of sources:
//!
//! * String literals
//! * Files: TOML, JSON
//! * Manual/programmatic override
//!
//! It also lets you define custom configuration sources, either "sync" or "async".
//!
//! ## Usage
//!
//! A simple usage example of the most basic use of this crate looks like this:
//!
//! ```rust
//! use config_rs_ng::Config;
//! use config_rs_ng::JsonFormatParser;
//! use config_rs_ng::StringSource;
//!
//! const CONFIGURATION: &str = r#"{"key": "value"}"#;
//!
//! // Define a source of configuration. In this case we use a `String` as source and specify that
//! // it should be parsed as JSON
//! let config_source = StringSource::<JsonFormatParser>::new(CONFIGURATION.to_string())
//!     .expect("building StringSource");
//!
//! // Build a `Config` object by loading the previously defined source
//! let config = Config::builder()
//!     .load(Box::new(config_source))
//!     .build()
//!     .expect("Building configuration object");
//!
//! // Accessing the config layers at "key"
//! let key = config
//!     .layers()
//!     .get("key")
//!     .expect("Accessing configuration object")
//!     .expect("Finding 'key' in configuration object");
//!
//! println!("'key' Config element is: '{:?}'", key);
//! ```
//!
//! For more complex examples, have a look at the `/examples` or the `/tests` folder of the
//! project.
//!
//! ## Accessing values
//!
//! The configuration is accessed (`config.get("key")` in the above example) via an object of a
//! type that can be parsed into an [Accessor](crate::Accessor) via
//! [ParsableAccessor](crate::ParsableAccessor).
//!
//! `config_rs_ng` implements [ParsableAccessor](crate::ParsableAccessor) for `&str` and `String`
//! for your convenience, but you can implement that trait for your custom accessors as well.
//! That implementation is rather dumb, the syntax is:
//!
//! * Elements are seperated via dot `.`
//! * If an element can be parsed as a number, it is an index into a list of values
//! * Otherwise it is a field-accessor to a map
//!
//! Example: `"foo.5.bar"` accesses a table at field "foo", expects a list which it accesses at
//! index 5 and then expects a table which it tries to access at "bar".
//!
//! If you want to access a specific value frequently, it might be a good idea to store your
//! [Accessor](crate::Accessor) object (to avoid re-constructing it all the time) and using
//! [Layers::get_with_accessor](crate::Layers::get_with_accessor) instead of
//! [Layers::get](crate::Layers::get).
//!
//! # Value metadata
//!
//! If you use [Layers::get_view](crate::Layers::get_view)
//! instead of [Layers::get](crate::Layers::get), you will (on success) get a
//! [ConfigView](crate::ConfigView).
//!
//! This object contains meta information for the configuration value you're trying to access.
//! Right now, it only contains a [ConfigSourceDescription](crate::ConfigSourceDescription), which
//! describes from what source the value comes from (e.g. a `PathBuf`).
//!
//! Later, span information might be supported as well (TODO).
//!

mod accessor;
mod config;
mod description;
mod element;
mod object;
mod source;

pub use config_rs_ng_derive::ConfigConstructor;
pub use config_rs_ng_derive::FromConfigElement;

pub use crate::accessor::AccessType;
pub use crate::accessor::Accessor;
pub use crate::accessor::ParsableAccessor;
#[cfg(feature = "async")]
pub use crate::config::AsyncConfig;
#[cfg(feature = "async")]
pub use crate::config::AsyncConfigBuilder;
pub use crate::config::Config;
pub use crate::config::ConfigBuilder;
pub use crate::config::ConfigConstructor;
pub use crate::config::FromConfigElement;
pub use crate::config::FromConfigElementError;
pub use crate::config::Layers;
pub use crate::description::ConfigSourceDescription;
pub use crate::element::ConfigElement;
pub use crate::element::ConfigElementListType;
pub use crate::element::ConfigElementMapType;
pub use crate::object::ConfigObject;
pub use crate::object::ConfigView;
pub use crate::source::ConfigSource;
pub use crate::source::FileSource;
pub use crate::source::FormatParser;
pub use crate::source::SourceError;
pub use crate::source::StringSource;

#[cfg(feature = "json")]
pub use crate::source::JsonFormatParser;

#[cfg(feature = "toml")]
pub use crate::source::TomlFormatParser;
