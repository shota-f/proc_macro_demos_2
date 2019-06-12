#![recursion_limit = "128"]

extern crate proc_macro;
use self::proc_macro::TokenStream as TS;

use quote::{quote, quote_spanned};
use syn::parse::{Parse, ParseStream, Result as ParseResult};
use syn::spanned::Spanned;
use syn::{parse_macro_input, Expr, Ident, Token, Type, Visibility};

struct LazyStatic {
    visibility: Visibility,
    name: Ident,
    ty: Type,
    init: Expr,
}

impl Parse for LazyStatic {
    fn parse(input: ParseStream) -> ParseResult<Self> {
        let visibility = input.parse::<Visibility>()?;
        input.parse::<Token! {static}>()?;
        input.parse::<Token! {ref}>()?;
        let name = input.parse::<Ident>()?;
        input.parse::<Token! {:}>()?;
        let ty = input.parse::<Type>()?;
        input.parse::<Token! {=}>()?;
        let init = input.parse::<Expr>()?;
        input.parse::<Token! {;}>()?;
        Ok(LazyStatic {
            visibility,
            name,
            ty,
            init,
        })
    }
}

#[proc_macro]
pub fn lazy_static(input: TS) -> TS {
    let LazyStatic {
        visibility,
        name,
        ty,
        init,
    } = parse_macro_input! { input as LazyStatic };

    let assert_sync = quote_spanned! { ty.span() => {
        struct _AssertSync
            where #ty: std::marker::Sync;
    }};

    let assert_sized = quote_spanned! { ty.span() => {
        struct _AssertSized
            where #ty: std::marker::Sized;
    }};

    let init_ptr = quote_spanned! { init.span() => {
        Box::into_raw(Box::new(#init))
    }};

    let expanded = quote! {
        #visibility struct #name;

        impl std::ops::Deref for #name {
            type Target = #ty;

            fn deref(&self) -> &#ty {
                #assert_sync
                #assert_sized

                static ONCE: std::sync::Once = std::sync::ONCE_INIT;
                static mut VALUE: *mut #ty = 0 as *mut #ty;

                unsafe {
                    ONCE.call_once(|| VALUE = #init_ptr);
                    &*VALUE
                }
            }
        }
    };

    expanded.into()
}