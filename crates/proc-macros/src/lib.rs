extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Pipeline)]
pub fn derive_pipeline(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let ident = &input.ident;

    let gen = quote! {
        impl render::Pipeline for #ident {
            fn box_eq(&self, other: &dyn Any) -> bool {
                other.downcast_ref::<Self>().map_or(false, |a| self == a)
            }
            fn as_any(&self) -> &dyn Any {
                self
            }
            fn clone_box(&self) -> Box<dyn render::Pipeline> {
                Box::new(self.clone())
            }
        }
    };

    TokenStream::from(gen)
}

#[proc_macro_derive(Property)]
pub fn derive_property(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let ident = &input.ident;

    let gen = quote! {
        impl #ident {
            /// Returns the value of a property.
            pub fn get(key: &str, entity: Entity, store: &StringComponentStore) -> $type {
                get_property::<$property>(key, entity, store).0
            }
        }

        impl IntoPropertySource<#ident> for #ident {
            fn into_source(self) -> PropertySource<#ident> {
                PropertySource::Value(self)
            }
        }
    };

    TokenStream::from(gen)
}
