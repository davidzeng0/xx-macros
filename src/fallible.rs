use std::fmt::Display;

use syn::parse::Parse;

use super::*;

pub fn try_expand<F>(func: F) -> TokenStream
where
	F: FnOnce() -> Result<TokenStream>
{
	match func() {
		Ok(tokens) => tokens,
		Err(err) => err.to_compile_error()
	}
}

#[must_use]
pub fn error_on_tokens<T, M, R>(tokens: T, message: M) -> R
where
	T: ToTokens,
	M: Display,
	R: Parse
{
	let error = Error::new_spanned(tokens, message).to_compile_error();

	parse_quote! { #error }
}
