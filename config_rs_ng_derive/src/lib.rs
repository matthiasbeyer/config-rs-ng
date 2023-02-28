use proc_macro::TokenStream as TS;

mod config_constructor;
mod from_config_element;

#[proc_macro_derive(ConfigConstructor)]
pub fn derive_config_constructor(input: TS) -> TS {
    crate::config_constructor::derive_config_constructor_impl(input)
}

#[proc_macro_derive(FromConfigElement)]
pub fn derive_from_config_element(input: TS) -> TS {
    crate::from_config_element::derive_from_config_element_impl(input)
}

