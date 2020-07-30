extern crate proc_macro;

use case::CaseExt;
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

    let helper = ident.to_string().as_str().to_snake();
    let helper = syn::Ident::new(helper.as_str(), Span::call_site());

    let name = syn::Ident::new(format!("{}Ctx", ident).as_str(), Span::call_site());
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
                                let setter = Ident::new(
                                    format!("set_{}", field_name).as_str(),
                                    Span::call_site(),
                                );

                                let clone = Ident::new(
                                    format!("clone_{}", field_name).as_str(),
                                    Span::call_site(),
                                );

                                let get_mut = Ident::new(
                                    format!("{}_mut", field_name).as_str(),
                                    Span::call_site(),
                                );

                                let gen = quote! {
                                    /// Gets a reference of the property value.
                                    #[inline(always)]
                                    pub fn #field_name(&self) -> &#ty {
                                        self.ctx.get::<#ty>(#field_name_str)
                                    }

                                    /// Gets a mutable reference of the property value.
                                    #[inline(always)]
                                    pub fn #get_mut(&mut self) -> &mut #ty {
                                        self.ctx.get_mut::<#ty>(#field_name_str)
                                    }

                                    /// Sets the property value.
                                    #[inline(always)]
                                    pub fn #setter(&mut self, value: impl Into<#ty>) {
                                        self.ctx.set(#field_name_str, value.into());
                                    }

                                    /// Clones the property value.
                                    #[inline(always)]
                                    pub fn #clone(&mut self) -> #ty {
                                        self.ctx.clone(#field_name_str)
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
        /// Represents a widget context that provides methods to access the properties of a widget.
        pub struct #name<'a> {
            ctx: WidgetContainer<'a>
        }

        /// Gets a context wrapper to access the properties of the widget.
        pub fn #helper(ctx: WidgetContainer<'_>) -> #name {
            #ident::get(ctx)
        }

        impl<'a> #name<'a> {
            #(#generated)*

            /// Update all properties from theme for the current widget.
            fn update(&mut self, force: bool) {
                self.ctx.update(force);
            }
        }

        impl #ident {
            /// Gets a widget context that wraps the given widgets an provides access to the its properties.
            pub fn get<'a>(ctx: WidgetContainer<'a>) -> #name<'a> {
                if *ctx.get::<TypeId>("type_id") != TypeId::of::<#ident>() {
                    let type_name = ctx.clone::<String>("type_name");
                    panic!("Wrong widget type {} for entity {:?} with type: {}.",
                        std::any::type_name::<#ident>(), ctx.entity(),
                        type_name);
                }

                #name {
                    ctx
                }
            }
        }
    };

    TokenStream::from(gen)
}
