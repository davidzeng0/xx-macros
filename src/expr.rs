use std::fmt::Display;
use std::str::FromStr;

use super::*;

macro_rules! as_lit_str_impl {
	($self:ident) => {
		match $self {
			Self::Lit(ExprLit { lit: Lit::Str(str), .. }) => Ok(str),
			_ => Err(Error::new_spanned($self, "Expected a string literal"))
		}
	};
}

sealed_ext! {
	impl Expr for Expr {
		fn as_lit_str(&self) -> Result<&LitStr> {
			as_lit_str_impl!(self)
		}

		fn as_lit_str_mut(&mut self) -> Result<&mut LitStr> {
			as_lit_str_impl!(self)
		}

		fn parse_lit_str<P, U>(&self, error: U) -> Result<P>
		where
			P: FromStr,
			U: Display
		{
			let str = self.as_lit_str()?;

			match str.value().parse() {
				Ok(parsed) => Ok(parsed),
				Err(_) => Err(Error::new_spanned(str, error))
			}
		}
	}

}
