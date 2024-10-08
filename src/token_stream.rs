use super::*;

sealed_ext! {
	impl TokenStream for TokenStream {
		fn require_empty(&self) -> Result<()> {
			if self.is_empty() {
				Ok(())
			} else {
				Err(Error::new_spanned(self, "Unexpected tokens"))
			}
		}
	}
}
