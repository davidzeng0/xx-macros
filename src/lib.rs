#![allow(clippy::wildcard_imports)]

use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::*;
use xx_macro_support::attribute::*;
use xx_macro_support::fallible::*;

mod strings;

#[proc_macro_attribute]
pub fn strings(
	attr: proc_macro::TokenStream, item: proc_macro::TokenStream
) -> proc_macro::TokenStream {
	strings::strings(attr.into(), item.into()).into()
}
