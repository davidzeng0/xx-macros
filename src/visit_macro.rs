use super::*;

pub fn visit_macro_body<V>(v: &mut V, mac: &mut Macro)
where
	V: VisitMut
{
	macro_rules! try_parse {
		($parse:expr, $($visit:tt)+) => {
			#[expect(clippy::redundant_closure_call)]
			if let Ok(parsed) = mac.parse_body_with($parse) {
				let mut visit = $($visit)*;

				mac.tokens = visit(parsed);

				return;
			}
		}
	}

	try_parse!(
		Punctuated::<Expr, Token![,]>::parse_terminated,
		|mut exprs: Punctuated<Expr, Token![,]>| {
			for expr in &mut exprs {
				v.visit_expr_mut(expr);
			}

			exprs.to_token_stream()
		}
	);

	try_parse!(Block::parse_within, |mut stmts: Vec<Stmt>| {
		for stmt in &mut stmts {
			v.visit_stmt_mut(stmt);
		}

		quote! { #(#stmts)* }
	});
}
