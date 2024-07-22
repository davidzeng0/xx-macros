use std::collections::HashSet;

use super::*;

#[derive(Default)]
struct Options {
	defaults: bool,
	display: bool,
	case: Option<Case>
}

impl Options {
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

impl Parse for Options {
	fn parse(input: ParseStream<'_>) -> Result<Self> {
		let args = Punctuated::<Ident, Token![,]>::parse_terminated(input)?;
		let mut this = Self::default();

		for arg in &args {
			match arg.to_string().as_ref() {
				"defaults" => this.set_default(arg)?,
				"display" => this.set_display(arg)?,
				"lowercase" => this.set_case(arg, Case::Flat)?,
				"snake" => this.set_case(arg, Case::Snake)?,
				_ => return Err(Error::new_spanned(arg, "Unknown option"))
			}
		}

		Ok(this)
	}
}

fn get_string(expr: &Expr, set: &mut HashSet<String>) -> Result<String> {
	let Expr::Lit(ExprLit { lit: Lit::Str(str), .. }) = expr else {
		return Err(Error::new_spanned(expr, "Expected a string"));
	};

	let value = str.value();

	if !set.insert(value.clone()) {
		return Err(Error::new_spanned(str, "Duplicate string"));
	}

	Ok(value)
}

fn expand(args: TokenStream, item: TokenStream) -> Result<TokenStream> {
	let options = parse2::<Options>(args)?;
	let mut item = parse2::<ItemEnum>(item)?;
	let mut variants = Vec::new();
	let mut alts = Vec::new();
	let mut strings = Vec::new();
	let mut set = HashSet::new();

	for variant in &mut item.variants {
		if remove_attr_path(&mut variant.attrs, "omit").is_some() {
			continue;
		}

		if !variant.fields.is_empty() {
			return Err(Error::new_spanned(variant, "Fields not allowed"));
		}

		let string = match remove_attr_name_value(&mut variant.attrs, "string") {
			Some(attr) => get_string(&attr.value, &mut set)?,
			None => {
				if !options.defaults {
					continue;
				}

				let mut result = variant.ident.to_string();

				if let Some(case) = options.case {
					result = result.to_case(case);
				}

				if !set.insert(result.clone()) {
					return Err(Error::new_spanned(&variant.ident, "Duplicate string"));
				}

				result
			}
		};

		while let Some(alt) = remove_attr_name_value(&mut variant.attrs, "alt") {
			let variant = &variant.ident;
			let str = get_string(&alt.value, &mut set)?;

			alts.push(quote! { #str => Ok(Self::#variant), });
		}

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
					#(#alts)*
					_ => Err(())
				}
			}
		}
	})
}

pub fn strings(attr: TokenStream, item: TokenStream) -> TokenStream {
	try_expand(|| expand(attr, item))
}
