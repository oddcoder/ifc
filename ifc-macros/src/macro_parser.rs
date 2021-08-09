use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::parse::{Parse, ParseStream, Result};
use syn::{punctuated::Punctuated, token::Comma, Expr};

pub struct MacroArgs {
    pub args: Punctuated<Expr, Comma>,
}

impl Parse for MacroArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(MacroArgs {
            args: input.parse_terminated(Expr::parse)?,
        })
    }
}

impl ToTokens for MacroArgs {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        self.args.to_tokens(tokens);
    }
}
