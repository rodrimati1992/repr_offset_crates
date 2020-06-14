#![recursion_limit = "192"]
#![deny(unused_variables)]
#![deny(unused_imports)]
#![deny(unused_parens)]
#![deny(unused_assignments)]
#![deny(unused_mut)]
#![deny(unreachable_patterns)]
#![deny(unused_doc_comments)]
#![deny(unconditional_recursion)]
#![deny(rust_2018_idioms)]
// The name of this lint is wrong,
// there's nothing redundant about using pattern matching instead of a method call
#![allow(clippy::redundant_pattern_matching)]
// I use `_` patterns to ensure that all fields are matched,
// using `..` would defeat the purpose for destructuring in the first place.
#![allow(clippy::unneeded_field_pattern)]
#![deny(clippy::shadow_unrelated)]
#![deny(clippy::wildcard_imports)]

#[allow(unused_extern_crates)]
extern crate proc_macro;

mod derive_repr_offset;

////////////////////////////////////////////////////////////////////////////////

use proc_macro::TokenStream as TokenStream1;

#[proc_macro_derive(ReprOffset, attributes(roff))]
pub fn derive_stable_abi(input: TokenStream1) -> TokenStream1 {
    syn::parse(input)
        .and_then(derive_repr_offset::derive)
        .unwrap_or_else(|e| e.to_compile_error())
        .into()
}

////////////////////////////////////////////////////////////////////////////////

#[cfg(all(test, not(feature = "testing")))]
compile_error! { "tests must be run with the \"testing\" feature" }
