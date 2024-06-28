use convert_case::*;
use proc_macro2::TokenStream;
use quote::*;
use syn::parse::*;
use syn::punctuated::*;
use syn::*;
use xx_macro_support::attribute::*;
use xx_macro_support::fallible::*;

#[derive(Default)]
struct StringsOptions {
	defaults: bool,
	display: bool,
	case: Option<Case>
}

impl StringsOptions {
	fn set_bool(opt: &mut bool, arg: &Ident) -> Result<()> {
		if *opt {
			Err(Error::new_spanned(arg, "Duplicate option"))
		} else {
			*opt = true;

			Ok(())
		}
	}

	fn set_default(&mut self, arg: &Ident) -> Result<()> {
		Self::set_bool(&mut self.defaults, arg)
	}

	fn set_display(&mut self, arg: &Ident) -> Result<()> {
		Self::set_bool(&mut self.display, arg)
	}

	fn set_case(&mut self, arg: &Ident, case: Case) -> Result<()> {
		if self.case.is_some() {
			Err(Error::new_spanned(arg, "Duplicate case option"))
		} else {
			self.case = Some(case);

			Ok(())
		}
	}
}

fn expand_strings(args: TokenStream, item: TokenStream) -> Result<TokenStream> {
	let args = Punctuated::<Ident, Token![,]>::parse_terminated.parse2(args)?;
	let mut options = StringsOptions::default();

	for arg in &args {
		match arg.to_string().as_ref() {
			"defaults" => options.set_default(arg)?,
			"display" => options.set_display(arg)?,
			"lowercase" => options.set_case(arg, Case::Flat)?,
			"snake" => options.set_case(arg, Case::Snake)?,
			_ => return Err(Error::new_spanned(arg, "Unknown option"))
		}
	}

	let mut item = parse2::<ItemEnum>(item)?;
	let mut variants = Vec::new();
	let mut strings = Vec::new();

	for variant in &mut item.variants {
		if !variant.fields.is_empty() {
			return Err(Error::new_spanned(variant, "Fields not allowed"));
		}

		if remove_attr_path(&mut variant.attrs, "omit").is_some() {
			continue;
		}

		let string = match remove_attr_name_value(&mut variant.attrs, "string") {
			Some(attr) => {
				let Expr::Lit(ExprLit { lit: Lit::Str(str), .. }) = &attr.value else {
					return Err(Error::new_spanned(attr, "Expected a string"));
				};

				str.value()
			}

			None => {
				if !options.defaults {
					continue;
				}

				let mut result = variant.ident.to_string();

				if let Some(case) = options.case {
					result = result.to_case(case);
				}

				result
			}
		};

		variants.push(variant.ident.clone());
		strings.push(string);
	}

	let name = &item.ident;
	let mut display = None;
	let as_str = if variants.len() == item.variants.len() {
		if options.display {
			display = Some(quote! {
				impl ::std::fmt::Display for #name {
					fn fmt(&self, fmt: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
						fmt.write_str(self.as_str())
					}
				}
			});
		}

		quote! {
			pub fn as_str(&self) -> &'static str {
				match self {
					#(Self::#variants => #strings),*
				}
			}
		}
	} else {
		quote! {
			pub fn as_str(&self) -> ::std::option::Option<&'static str> {
				use ::std::option::Option::*;

				match self {
					#(Self::#variants => Some(#strings),)*
					_ => None
				}
			}
		}
	};

	Ok(quote! {
		#item

		impl #name {
			#[must_use]
			#as_str
		}

		#display

		impl ::std::str::FromStr for #name {
			type Err = ();

			fn from_str(str: &str) -> ::std::result::Result<Self, ()> {
				use ::std::result::Result::*;

				match str {
					#(#strings => Ok(Self::#variants),)*
					_ => Err(())
				}
			}
		}
	})
}

#[proc_macro_attribute]
pub fn strings(
	attr: proc_macro::TokenStream, item: proc_macro::TokenStream
) -> proc_macro::TokenStream {
	try_expand(|| expand_strings(attr.into(), item.into())).into()
}
