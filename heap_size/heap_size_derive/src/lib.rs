extern crate proc_macro;

use proc_macro::TokenStream as TS;
use proc_macro2::TokenStream as TS2;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;
use syn::{
    parse_macro_input, parse_quote, Data, DeriveInput, Fields, GenericParam, Generics, Index,
};

#[proc_macro_derive(HeapSize)]
pub fn derive_heap_size(input: TS) -> TS {
    let input = parse_macro_input! { input as DeriveInput };
    let struct_name = input.ident;

    let generics = add_trait_bounds(input.generics);
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    let sum = heap_size_sum(&input.data);

    let expanded = quote! {
        impl #impl_generics heap_size::HeapSize for #struct_name #ty_generics #where_clause {
            fn heap_size_of_children(&self) -> usize {
                #sum
            }
        }
    };

    expanded.into()
}

fn add_trait_bounds(mut generics: Generics) -> Generics {
    for param in generics.params.iter_mut() {
        if let GenericParam::Type(type_param) = param {
            type_param.bounds.push(parse_quote! {heap_size::HeapSize});
        }
    }
    generics
}

fn heap_size_sum(data: &Data) -> TS2 {
    match *data {
        Data::Struct(ref data_struct) => match data_struct.fields {
            Fields::Named(ref fields) => {
                let recurse = fields.named.iter().map(|field| {
                    let field_name = &field.ident;
                    quote_spanned! { field.span() => {
                        heap_size::HeapSize::heap_size_of_children(&self . #field_name)
                    }}
                });
                quote! {
                    0 #( + #recurse)*
                }
            }
            Fields::Unnamed(ref fields) => {
                let recurse = fields.unnamed.iter().enumerate().map(|(i, field)| {
                    let index = Index::from(i);
                    quote_spanned! {field.span() => {
                        heap_size::HeapSize::heap_size_of_children(&self . #index)
                    }}
                });
                quote! {
                    0 #( + #recurse)*
                }
            }
            Fields::Unit => {
                quote! { 0 }
            }
        },
        Data::Enum(_) | Data::Union(_) => unimplemented! {},
    }
}