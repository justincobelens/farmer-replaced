extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(Resource)]
pub fn derive_resource(input: TokenStream) -> TokenStream {
	let ast: DeriveInput = parse_macro_input!(input);

	let name = &ast.ident;

	let expanded = quote! {
		impl Resource for #name {}
	};

	TokenStream::from(expanded)
}
