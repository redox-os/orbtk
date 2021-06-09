extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{parse_macro_input, DataStruct, DeriveInput, Ident, Meta, NestedMeta};

#[proc_macro_derive(Pipeline)]
pub fn derive_pipeline(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let ident = &input.ident;

    let gen = quote! {
        impl PipelineTrait for #ident {
            fn box_eq(&self, other: &dyn Any) -> bool {
                other.downcast_ref::<Self>().map_or(false, |a| self == a)
            }
            fn as_any(&self) -> &dyn Any {
                self
            }
            fn clone_box(&self) -> Box<dyn PipelineTrait> {
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

#[proc_macro_derive(IntoRenderObject)]
pub fn derive_into_render_object(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let ident = &input.ident;

    let gen = quote! {
        impl Into<Box<dyn RenderObject>> for #ident {
            fn into(self) -> Box<dyn RenderObject> {
                Box::new(self)
            }
        }
    };

    TokenStream::from(gen)
}

#[proc_macro_derive(IntoLayout)]
pub fn derive_into_layout(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let ident = &input.ident;

    let gen = quote! {
        impl Into<Box<dyn Layout>> for #ident {
            fn into(self) -> Box<dyn Layout> {
                Box::new(self)
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

#[proc_macro_derive(WidgetCtx, attributes(property))]
pub fn derive_widget_ctx(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let ident = &input.ident;

    let mut generated = vec![];

    if let syn::Data::Struct(DataStruct { ref fields, .. }) = input.data {
        let filter_fields = fields
            .iter()
            .filter(|f| f.attrs.iter().any(|attr| attr.path.is_ident("property")));

        for field in filter_fields {
            let field_name = field
                .clone()
                .ident
                .unwrap_or_else(|| panic!("Expected the field to have a name"));

            let field_name_str = field_name.to_string();

            for attr in field.attrs.iter() {
                let last_attr_path = attr
                    .path
                    .segments
                    .iter()
                    .last()
                    .expect("Expected at least one segment where #[segment[::segment*](..)]");
                if (*last_attr_path).ident != "property" {
                    continue;
                }

                let meta = match attr.parse_meta() {
                    Ok(meta) => meta,
                    Err(_) => continue,
                };
                let list = match meta {
                    Meta::List(l) => l,
                    _ if meta.path().is_ident("property") => {
                        panic!("Invalid #[new] attribute, expected #[property(..)]")
                    }
                    _ => continue,
                };

                for item in list.nested.iter() {
                    match *item {
                        NestedMeta::Meta(Meta::Path(ref path)) => {
                            let ty = path.get_ident();

                            if let Some(ty) = ty {
                                let getter = Ident::new(
                                    format!("{}_ref", field_name).as_str(),
                                    Span::call_site(),
                                );

                                let setter = Ident::new(
                                    format!("{}_set", field_name).as_str(),
                                    Span::call_site(),
                                );

                                let clone = Ident::new(
                                    format!("{}_clone", field_name).as_str(),
                                    Span::call_site(),
                                );

                                let get_mut = Ident::new(
                                    format!("{}_mut", field_name).as_str(),
                                    Span::call_site(),
                                );

                                let gen = quote! {
                                    /// Gets a reference of the property value. Panics if it is the wrong widget type.
                                    #[inline(always)]
                                    pub fn #getter<'a>(widget: &'a WidgetContainer<'a>) -> &'a #ty {
                                        #ident::panics_on_wrong_type(widget);
                                        widget.get(#field_name_str)
                                    }

                                    /// Gets a mutable reference of the property value. Panics if it is the wrong widget type.
                                    #[inline(always)]
                                    pub fn #get_mut<'a>(widget: &'a mut WidgetContainer<'a>) -> &'a mut #ty {
                                        #ident::panics_on_wrong_type(widget);
                                        widget.get_mut(#field_name_str)
                                    }

                                    /// Sets the property value. Panics if it is the wrong widget type.
                                    #[inline(always)]
                                    pub fn #setter(widget: &mut WidgetContainer, value: impl Into<#ty>) {
                                        #ident::panics_on_wrong_type(widget);
                                        widget.set(#field_name_str, value.into());
                                    }

                                    /// Clones the property value. Panics if it is the wrong widget type.
                                    #[inline(always)]
                                    pub fn #clone(widget: &WidgetContainer) -> #ty {
                                        widget.clone(#field_name_str)
                                    }
                                };

                                generated.push(gen);
                            }

                            break;
                        }
                        _ => continue,
                    }
                }
            }
        }
    }

    let gen = quote! {
        impl #ident {
            #(#generated)*
        }
    };

    TokenStream::from(gen)
}
