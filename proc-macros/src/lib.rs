use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use xx_macros::*;

mod strings;

declare_attribute_macro! {
	pub fn strings(attr: TokenStream, item: TokenStream) -> Result<TokenStream> {
		strings::strings(attr, item)
	}
}
