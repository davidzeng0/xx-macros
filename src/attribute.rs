use super::*;

sealed_ext! {
	impl Attributes for Vec<Attribute> {
		fn remove_if(&mut self, matches: impl Fn(&Attribute) -> bool) -> Option<Attribute> {
			let index = self.iter().position(matches)?;

			Some(self.remove(index))
		}

		fn remove_ident_if(
			&mut self, target: &str, kind_match: impl Fn(&Meta) -> bool
		) -> Option<Attribute> {
			self.remove_if(|attr| {
				if !kind_match(&attr.meta) {
					return false;
				}

				attr.path().get_ident().is_some_and(|ident| ident == target)
			})
		}

		fn remove_any(&mut self, target: &str) -> Option<Attribute> {
			self.remove_ident_if(target, |_| true)
		}

		fn remove_path(&mut self, target: &str) -> Option<Attribute> {
			self.remove_ident_if(target, |meta| matches!(meta, Meta::Path(_)))
		}

		fn remove_name_value(&mut self, target: &str) -> Option<MetaNameValue> {
			let attr = self.remove_ident_if(target, |meta| matches!(meta, Meta::NameValue(_)))?;

			Some(match attr.meta {
				Meta::NameValue(nv) => nv,
				_ => unreachable!()
			})
		}

		fn remove_list(&mut self, target: &str) -> Option<MetaList> {
			let attr = self.remove_ident_if(target, |meta| matches!(meta, Meta::List(_)))?;

			Some(match attr.meta {
				Meta::List(list) => list,
				_ => unreachable!()
			})
		}
	}
}
