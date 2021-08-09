#![cfg_attr(
    feature = "nightly",
    feature(proc_macro_diagnostic, proc_macro_span, proc_macro_def_site)
)]

mod attributes;
mod ctxt;
mod error;
mod macro_parser;
mod scope;
mod semi_block;

use ctxt::IfcContext;
use macro_parser::MacroArgs;
use proc_macro::TokenStream;
use proc_macro_error::*;
use quote::quote;
use semi_block::SemiBlock;
use syn::parse_macro_input;
use syn::spanned::Spanned;

#[proc_macro_error]
#[proc_macro]
pub fn ifc_block(tokens: TokenStream) -> TokenStream {
    let mut syn_tree = parse_macro_input!(tokens as SemiBlock);
    let mut ctxt = IfcContext::new(syn_tree.span());
    for stmt in syn_tree.stmts.iter_mut() {
        ctxt.process_stmt(stmt)
    }
    quote!(
        #syn_tree
    )
    .into()
}
