#![cfg_attr(
    feature = "nightly",
    feature(proc_macro_diagnostic, proc_macro_span, proc_macro_def_site)
)]

mod attributes;
mod ctxt;
mod error;
mod semi_block;

use ctxt::IfcContext;
use proc_macro::TokenStream;
use proc_macro_error::*;
use quote::quote;
use semi_block::SemiBlock;
use syn::parse_macro_input;

#[proc_macro_error]
#[proc_macro]
pub fn ifc_block(tokens: TokenStream) -> TokenStream {
    let mut syn_tree = parse_macro_input!(tokens as SemiBlock);
    let mut ctxt = IfcContext::new();
    for stmt in syn_tree.stmts.iter_mut() {
        ctxt.process_stmt(stmt)
    }
    quote!(
        #syn_tree
    )
    .into()
}
