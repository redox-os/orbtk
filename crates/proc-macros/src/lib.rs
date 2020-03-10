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

#[proc_macro_derive(AsAny)]
pub fn derive_as_any(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let ident = &input.ident;

    let gen = quote! {
        impl AsAny for #ident {
            fn as_any(&self) -> &dyn Any {
                self
            }

            fn as_any_mut(&mut self) -> &mut dyn Any {
                self
            }
        }
    };

    TokenStream::from(gen)
}

#[proc_macro_derive(Event)]
pub fn derive_event(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let ident = &input.ident;

    let gen = quote! {
        impl Event for #ident {}
    };

    TokenStream::from(gen)
}

#[proc_macro_derive(IntoHandler)]
pub fn derive_into_handler(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let ident = &input.ident;

    let gen = quote! {
        impl Into<Rc<dyn EventHandler>> for #ident {
            fn into(self) -> Rc<dyn EventHandler> {
                Rc::new(self)
            }
        }
    };

    TokenStream::from(gen)
}
