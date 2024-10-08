use super::*;

#[expect(missing_copy_implementations)]
pub struct RemoveModifiers;

impl VisitMut for RemoveModifiers {
	fn visit_pat_ident_mut(&mut self, ident: &mut PatIdent) {
		ident.by_ref = None;
		ident.mutability = None;
	}
}

sealed_ext! {
	impl ReturnType for ReturnType {
		fn to_type(&self) -> Type {
			if let Self::Type(_, ty) = self {
				ty.as_ref().clone()
			} else {
				parse_quote! { () }
			}
		}
	}
}

sealed_ext! {
	impl FunctionArgs for Punctuated<FnArg, Token![,]> {
		#[must_use]
		fn get_pats(&self, include_receiver: bool) -> Punctuated<Expr, Token![,]> {
			let mut args = Punctuated::new();

			for arg in self {
				match arg {
					FnArg::Typed(arg) => {
						let mut pat = arg.pat.as_ref().clone();

						RemoveModifiers.visit_pat_mut(&mut pat);

						args.push(parse_quote! { #pat });
					}

					FnArg::Receiver(rec) => {
						if include_receiver {
							let token = &rec.self_token;

							args.push(parse_quote! { #token });
						}
					}
				}
			}

			args
		}
	}
}
