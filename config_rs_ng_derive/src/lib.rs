use proc_macro::TokenStream as TS;

mod config_constructor;

#[proc_macro_derive(ConfigConstructor)]
pub fn derive_config_constructor(input: TS) -> TS {
    crate::config_constructor::derive_config_constructor_impl(input)
}
