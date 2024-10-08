pub use paste::paste;

use super::*;

#[macro_export]
macro_rules! sealed_trait {
	($($name:ident)?) => {
		$crate::macros::paste! {
			#[allow(clippy::allow_attributes, non_snake_case)]
			mod [< __private_seal_ $($name)? >] {
				pub trait [< $($name)? Sealed >] {}
			}

			use [< __private_seal_ $($name)? >]::[< $($name)? Sealed >];
		}
	};

	(trait $trait:ident) => {
		$crate::macros::paste! {
			#[allow(non_snake_case)]
			mod [< __private_seal_ $trait >] {
				pub trait [< $trait Sealed >]: super::$trait {}

				impl<T: super::$trait> [< $trait Sealed >] for T {}
			}

			use [< __private_seal_ $trait >]::[< $trait Sealed >];
		}
	};

	(($($tokens:tt)*)) => {
		$crate::macros::sealed_trait!($($tokens)*);
	}
}

pub use sealed_trait;

#[macro_export]
macro_rules! sealed_ext {
	{
		impl
		$(< $($generics:tt)* >)?
		$seal:ident for
		$what:ty {
			$(
				$(#$attrs:tt)*
				fn $func:ident $(< $($func_generics:ident),* >)? ($($args:tt)*) $(-> $ret:ty)?
				$(where $($where_generic:ident: $($bounds:ident)+),*)?
				$block:block
			)*
		}
	} => {
		$crate::macros::sealed_trait!($seal);

		$crate::macros::paste! {
			pub trait [<$seal Ext>]: [<$seal Sealed>] {
				$(
					$(#$attrs)*
					fn $func $(< $($func_generics),* >)? ($($args)*) $(-> $ret)?
					$(where $($where_generic: $($bounds)+),*)?;
				)*
			}

			impl $(< $($generics)* >)? [<$seal Sealed>] for $what {}

			impl $(< $($generics)* >)? [<$seal Ext>] for $what {
				$(
					$(#$attrs)*
					fn $func $(<$($func_generics),*>)? ($($args)*) $(-> $ret)?
					$(where $($where_generic: $($bounds)+),*)?
					$block
				)*
			}
		}
	};
}

pub use sealed_ext;

#[doc(hidden)]
#[macro_export]
macro_rules! attribute_macro_header {
	($(#$attrs: tt)*, $ident:ident, $attr:ident, $item:ident, $block:block) => {
		#[proc_macro_attribute]
		$(#$attrs)*
		pub fn $ident(
			$attr: proc_macro::TokenStream, $item: proc_macro::TokenStream
		) -> proc_macro::TokenStream $block
	}
}

#[doc(hidden)]
pub use attribute_macro_header;

#[macro_export]
macro_rules! declare_attribute_macro {
	{
		$(#$attrs: tt)*
		pub fn
		$ident:ident($attr:ident : TokenStream, $item:ident : TokenStream) -> TokenStream
		$block:block
	} => {
		$crate::macros::attribute_macro_header!($(#$attrs)*, $ident, $attr, $item, {
			fn $ident($attr: TokenStream, $item: TokenStream) -> TokenStream $block

			$ident($attr.into(), $item.into()).into()
		});
	};

	{
		$(#$attrs: tt)*
		pub fn
		$ident:ident($attr:ident : TokenStream, $item:ident : TokenStream) -> Result<TokenStream>
		$block:block
	} => {
		$crate::macros::attribute_macro_header!($(#$attrs)*, $ident, $attr, $item, {
			fn $ident($attr: TokenStream, $item: TokenStream) -> Result<TokenStream> $block

			$crate::fallible::try_expand(|| $ident($attr.into(), $item.into())).into()
		});
	};
}

pub use declare_attribute_macro;

#[doc(hidden)]
#[macro_export]
macro_rules! proc_macro_header {
	($(#$attrs: tt)*, $ident:ident, $item:ident, $block:block) => {
		#[proc_macro]
		$(#$attrs)*
		pub fn $ident($item: proc_macro::TokenStream) -> proc_macro::TokenStream $block
	}
}

#[doc(hidden)]
pub use proc_macro_header;

#[macro_export]
macro_rules! declare_proc_macro {
	{
		$(#$attrs: tt)*
		pub fn
		$ident:ident($item:ident : TokenStream) -> TokenStream
		$block:block
	} => {
		$crate::macros::proc_macro_header!($(#$attrs)*, $ident, $item, {
			fn $ident($item: TokenStream) -> TokenStream $block

			$ident($item.into()).into()
		});
	};

	{
		$(#$attrs: tt)*
		pub fn
		$ident:ident($item:ident : TokenStream) -> Result<TokenStream>
		$block:block
	} => {
		$crate::macros::proc_macro_header!($(#$attrs)*, $ident, $item, {
			fn $ident($item: TokenStream) -> Result<TokenStream> $block

			$crate::fallible::try_expand(|| $ident($item.into())).into()
		});
	};
}

pub use declare_proc_macro;

#[doc(hidden)]
#[macro_export]
macro_rules! derive_macro_header {
	($(#$attrs: tt)*, $ident:ident, $item:ident, $block:block) => {
		$(#$attrs)*
		pub fn $ident($item: proc_macro::TokenStream) -> proc_macro::TokenStream $block
	}
}

#[doc(hidden)]
pub use derive_macro_header;

#[macro_export]
macro_rules! declare_derive_macro {
	{
		$(#$attrs: tt)*
		pub fn
		$ident:ident($item:ident : TokenStream) -> TokenStream
		$block:block
	} => {
		$crate::macros::derive_macro_header!($(#$attrs)*, $ident, $item, {
			fn $ident($item: TokenStream) -> TokenStream $block

			$ident($item.into()).into()
		});
	};

	{
		$(#$attrs: tt)*
		pub fn
		$ident:ident($item:ident : TokenStream) -> Result<TokenStream>
		$block:block
	} => {
		$crate::macros::derive_macro_header!($(#$attrs)*, $ident, $item, {
			fn $ident($item: TokenStream) -> Result<TokenStream> $block

			$crate::fallible::try_expand(|| $ident($item.into())).into()
		});
	};
}

pub use declare_derive_macro;
