use proc_macro2::TokenStream;
use quote::{ToTokens, TokenStreamExt};
use syn::parse::{Parse, ParseStream, Result};
use syn::{Block, Stmt};

/// Block with braces
pub struct SemiBlock {
    pub stmts: Vec<Stmt>,
}

impl Parse for SemiBlock {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        Ok(SemiBlock {
            stmts: Block::parse_within(input)?,
        })
    }
}

impl ToTokens for SemiBlock {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append_all(&self.stmts);
    }
}
