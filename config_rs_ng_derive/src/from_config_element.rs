use proc_macro::TokenStream as TS;
use proc_macro2::Ident;
use proc_macro2::TokenStream;
use proc_macro_error::abort;
use quote::TokenStreamExt;
use syn::{parse_macro_input, DeriveInput, Type};

enum FieldConstruction<'q> {
    Named { name: Option<&'q Ident>, ty: &'q Type },
}

impl<'q> quote::ToTokens for FieldConstruction<'q> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            FieldConstruction::Named { name, ty } => {
                tokens.append_all({
                    quote::quote! {
                        #name: {
                            map.get(stringify!(#name))
                                .ok_or_else(|| config_rs_ng::FromConfigElementError::NoElement {
                                    name: stringify!(#name).to_string(),
                                    ty: stringify!(#ty).to_string(),
                                })
                            .and_then(#ty::from_config_element)?
                        },
                    }
                });
            }
        };
    }
}


pub fn derive_from_config_element_impl(input: TS) -> TS {
    let input = parse_macro_input!(input as DeriveInput);
    let ident = &input.ident;

    let construction: Vec<FieldConstruction> = if let syn::Data::Struct(data) = &input.data {
        match &data.fields {
            syn::Fields::Named(fields) => fields
                .named
                .iter()
                .map(|field| FieldConstruction::Named {
                    name: field.ident.as_ref(),
                    ty: &field.ty,
                })
                .collect::<Vec<FieldConstruction>>(),
            syn::Fields::Unnamed(_) => abort!(ident, "Unnamed fields are not supported"),
            syn::Fields::Unit => abort!(
                ident,
                "Unit structs are not supported as they cannot be represented"
            )
        }
    } else {
        abort!(
            ident,
            "Currently, only structs are supported"
        )
    };

    let expanded = quote::quote! {
        impl config_rs_ng::FromConfigElement for #ident {
            type Error = config_rs_ng::FromConfigElementError;

            fn from_config_element(element: &dyn config_rs_ng::ConfigElement) -> Result<Self, Self::Error> {
                let map = element.as_map().ok_or_else(|| {
                    let found = element.get_type().name();
                    config_rs_ng::FromConfigElementError::TypeError {
                        expected: "map",
                        found,
                    }
                })?;

                Ok({
                    Self {
                        #( #construction )*
                    }
                })
            }
        }
    };

    TS::from(expanded)
}
