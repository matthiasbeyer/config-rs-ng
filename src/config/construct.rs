use std::collections::BTreeMap;
use std::collections::HashMap;

use crate::ConfigElement;

use super::{ConfigError, Layers};

pub trait ConfigConstructor
where
    Self: Sized,
{
    type Error;

    fn construct_from(layers: &Layers) -> Result<Self, Self::Error>;
}

pub trait FromConfigElement
where
    Self: Sized,
{
    type Error;

    fn from_config_element(element: &dyn ConfigElement) -> Result<Self, Self::Error>;
}

#[derive(Debug, thiserror::Error)]
#[non_exhaustive]
pub enum FromConfigElementError {
    #[error(transparent)]
    Custom(#[from] Box<dyn std::error::Error>),

    #[error("Type error. Expected '{expected}', got '{found}'")]
    TypeError {
        expected: &'static str,
        found: &'static str,
    },

    #[error("Element for member {name}: {ty} not found")]
    NoElement { name: String, ty: String },

    #[error(transparent)]
    ConfigError(#[from] ConfigError),
}

macro_rules! impl_from_config_element {
    ($t:ty, $fun:tt, $tname:literal) => {
        impl FromConfigElement for $t {
            type Error = FromConfigElementError;

            fn from_config_element(element: &dyn ConfigElement) -> Result<Self, Self::Error> {
                element.$fun().ok_or_else(|| {
                    let found = element.get_type().name();
                    FromConfigElementError::TypeError {
                        expected: $tname,
                        found,
                    }
                })
            }
        }
    };
}

impl_from_config_element!(bool, as_bool, "bool");
impl_from_config_element!(i8, as_i8, "i8");
impl_from_config_element!(i16, as_i16, "i16");
impl_from_config_element!(i32, as_i32, "i32");
impl_from_config_element!(i64, as_i64, "i64");
impl_from_config_element!(u8, as_u8, "u8");
impl_from_config_element!(u16, as_u16, "u16");
impl_from_config_element!(u32, as_u32, "u32");
impl_from_config_element!(u64, as_u64, "u64");
impl_from_config_element!(f32, as_f32, "f32");
impl_from_config_element!(f64, as_f64, "f64");

impl FromConfigElement for String {
    type Error = FromConfigElementError;

    fn from_config_element(element: &dyn ConfigElement) -> Result<Self, Self::Error> {
        element.as_str().map(String::from).ok_or_else(|| {
            let found = element.get_type().name();
            FromConfigElementError::TypeError {
                expected: "str",
                found,
            }
        })
    }
}

impl<T> FromConfigElement for Option<T>
where
    T: FromConfigElement<Error = FromConfigElementError>,
{
    type Error = FromConfigElementError;

    fn from_config_element(element: &dyn ConfigElement) -> Result<Self, Self::Error> {
        if element.is_null() {
            Ok(None)
        } else {
            T::from_config_element(element).map(Some)
        }
    }
}

impl<T> FromConfigElement for Vec<T>
where
    T: FromConfigElement<Error = FromConfigElementError>,
{
    type Error = FromConfigElementError;

    fn from_config_element(element: &dyn ConfigElement) -> Result<Self, Self::Error> {
        let list = element.as_list().ok_or_else(|| {
            let found = element.get_type().name();
            FromConfigElementError::TypeError {
                expected: "list",
                found,
            }
        })?;

        let mut v = Vec::with_capacity(list.len());
        for index in 0..list.len() {
            v.push(T::from_config_element(list.at_index(index).unwrap())?);
        }
        Ok(v)
    }
}

impl<T> FromConfigElement for HashMap<String, T>
where
    T: FromConfigElement<Error = FromConfigElementError>,
{
    type Error = FromConfigElementError;

    fn from_config_element(element: &dyn ConfigElement) -> Result<Self, Self::Error> {
        let map = element.as_map().ok_or_else(|| {
            let found = element.get_type().name();
            FromConfigElementError::TypeError {
                expected: "map",
                found,
            }
        })?;

        map.keys()
            .into_iter()
            .map(|key| T::from_config_element(map.get(&key).unwrap()).map(|val| (key, val)))
            .collect::<Result<HashMap<String, T>, _>>()
    }
}

impl<T> FromConfigElement for BTreeMap<String, T>
where
    T: FromConfigElement<Error = FromConfigElementError>,
{
    type Error = FromConfigElementError;

    fn from_config_element(element: &dyn ConfigElement) -> Result<Self, Self::Error> {
        let map = element.as_map().ok_or_else(|| {
            let found = element.get_type().name();
            FromConfigElementError::TypeError {
                expected: "map",
                found,
            }
        })?;

        map.keys()
            .into_iter()
            .map(|key| T::from_config_element(map.get(&key).unwrap()).map(|val| (key, val)))
            .collect::<Result<BTreeMap<String, T>, _>>()
    }
}
