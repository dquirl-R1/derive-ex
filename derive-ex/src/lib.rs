#![allow(clippy::large_enum_variant)]

extern crate proc_macro;

#[macro_use]
mod syn_utils;

mod bound;
mod common;
mod item_impl;
mod item_type;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse2, Item, Result};

#[doc = include_str!("../../doc/derive_ex.md")]
#[proc_macro_attribute]
pub fn derive_ex(
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let mut item: TokenStream = item.into();
    match build(attr.into(), item.clone()) {
        Ok(s) => s,
        Err(e) => {
            item.extend(e.to_compile_error());
            item
        }
    }
    .into()
}

/// Use attribute macro [`macro@derive_ex`] as derive macro.
///
/// [`macro@derive_ex`], being an attribute macro designed to mimic the functionality of the derive macro,
/// may cause rust-analyzer's assistance to not work correctly in certain cases.
///
/// Adding `#[derive(Ex)]` to an item with `#[derive_ex]` will allow rust-analyzer's assistance to work correctly.
///
/// In the example below, without `#[derive(Ex)]`,
/// the jump from `value: String` to the definition of `String` is not possible,
/// but with `#[derive(Ex)]`, it is possible.
///
/// ```
/// use derive_ex::Ex;
///
/// #[derive(Ex)]
/// #[derive_ex(Eq, PartialEq)]
/// struct X {
///     #[eq(key = $.len())]
///     value: String,
/// }
/// ```
#[proc_macro_derive(
    Ex,
    attributes(derive_ex, ord, partial_ord, eq, partial_eq, hash, debug, default)
)]
pub fn derive_ex_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input: TokenStream = input.into();
    match item_type::build_derive(input) {
        Ok(s) => s,
        Err(e) => e.to_compile_error(),
    }
    .into()
}

fn build(attr: TokenStream, item: TokenStream) -> Result<TokenStream> {
    let mut item: Item = parse2(item)?;
    let ts = match &mut item {
        Item::Struct(item_struct) => item_type::build_by_item_struct(attr, item_struct),
        Item::Enum(item_enum) => item_type::build_by_item_enum(attr, item_enum),
        Item::Impl(item_impl) => item_impl::build_by_item_impl(attr, item_impl),
        _ => bail!(
            _,
            "`#[derive_ex]` can be specified only for `struct`, `enum`, or `impl`.",
        ),
    }
    .unwrap_or_else(|e| e.to_compile_error());

    Ok(quote!(#item #ts))
}
