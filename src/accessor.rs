/// Implemented by types that can be parsed to [Accessor]
///
/// Convenience-implementations exist for `&str` and `String`, but the parsing implementation is
/// rather simple. Users might want to roll their own parsers for their custom accessors for more
/// sophisticated accessor markup.
pub trait ParsableAccessor {
    fn parse(&self) -> Result<Accessor, AccessorParseError>;
}

impl ParsableAccessor for &str {
    fn parse(&self) -> Result<Accessor, AccessorParseError> {
        use std::str::FromStr;

        // TODO: Make this non-trivial and bulletproof

        let accessor = self
            .split('.')
            .map(|s| match usize::from_str(s) {
                Ok(u) => AccessType::Index(u),
                Err(_) => AccessType::Key(s.to_string()),
            })
            .collect();

        Ok(Accessor::new(accessor))
    }
}

impl ParsableAccessor for String {
    fn parse(&self) -> Result<Accessor, AccessorParseError> {
        let s: &str = self;
        ParsableAccessor::parse(&s)
    }
}

/// An object that can be used to grab into a [Layers] and get a value from it
///
/// An instance of this type can be used to access a configuration value from a [Layers] instance
/// (that one retrieves via [Config::layers]).
pub struct Accessor {
    stack: Vec<AccessType>,
    index: usize,
}

impl Accessor {
    pub fn new(stack: Vec<AccessType>) -> Self {
        Self { stack, index: 0 }
    }
}

pub enum AccessType {
    Key(String),
    Index(usize),
}

impl Accessor {
    pub(crate) fn current(&self) -> Option<&AccessType> {
        self.stack.get(self.index)
    }

    pub(crate) fn advance(&mut self) {
        self.index += 1;
    }
}

#[derive(Debug, thiserror::Error)]
pub enum AccessorParseError {}
