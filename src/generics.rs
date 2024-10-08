use std::mem::take;

use super::*;

sealed_ext! {
	impl WhereClause for WhereClause {
		fn default() -> Self {
			Self {
				where_token: Default::default(),
				predicates: Punctuated::new()
			}
		}
	}
}

sealed_ext! {
	impl Generics for Generics {
		fn remove_lifetimes(&mut self) {
			self.params = take(&mut self.params)
				.into_iter()
				.filter(|generic| !matches!(generic, GenericParam::Lifetime(_)))
				.collect();
		}

		fn to_types_turbofish(&self) -> TokenStream {
			let mut generics = self.clone();

			generics.remove_lifetimes();
			generics
				.split_for_impl()
				.1
				.as_turbofish()
				.into_token_stream()
		}
	}
}
