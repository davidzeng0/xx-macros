use convert_case::{Case, Casing};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use xx_macro_support::*;

mod strings;

declare_attribute_macro! {
	pub fn strings(attr: TokenStream, item: TokenStream) -> Result<TokenStream> {
		strings::strings(attr, item)
	}
}
