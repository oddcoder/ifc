use super::IfcContext;

use crate::attributes::{Attributes, VariableState};
use crate::error::{assign_high2low, high_guard};
use proc_macro2::Span;
use quote::quote;
use syn::spanned::Spanned;
use syn::{parse, Expr, Local, Pat, PatIdent, PatType, Type};

/// THis file is for processing let statement
/// We will refer to multiple statements here
/// 1- let x = 5;
/// 2- let x : u8 = 5;

impl IfcContext {
    /// This function processes "x" part of both statement 1 and 2
    fn process_pat_ident(&mut self, ident: &mut PatIdent, ifc_attrs: &Attributes) {
        assert!(
            ident.subpat.is_none(),
            "Subpatterns in identifiers are unimplemented"
        );
        match ifc_attrs.state.get() {
            VariableState::Low => self.add_low(ident.ident.clone()),
            VariableState::High => self.add_high(ident.ident.clone()),
            VariableState::None => unreachable!(),
        };
    }

    /// This function processes "x: u8" in second statement
    fn process_pat_types(&mut self, t: &mut PatType, ifc_attrs: &Attributes) {
        let old_type = &t.ty;
        // wrap the "u8" part with High variable or Low variable
        let new_type_tokens = match ifc_attrs.state.get() {
            VariableState::Low => quote!(ifc::LowVar<#old_type>),
            VariableState::High => quote!(ifc::HighVar<#old_type>),
            VariableState::None => unreachable!(),
        };
        // Embed the new type
        t.ty =
            Box::new(parse::<Type>(new_type_tokens.into()).expect(
                "Fatal Error: Ifc-macros had Quote generated rust code that failed to parse",
            ));

        // process the "x" part since it is the same with statement 1 and 2
        match &mut *t.pat {
            Pat::Ident(i) => {
                self.process_pat_ident(i, ifc_attrs);
            }
            _ => unimplemented!(),
        };
    }

    fn get_lhs_span(&mut self, local: &Local) -> Span {
        match &local.pat {
            Pat::Ident(i) => i.span(),
            Pat::Type(t) => t.pat.span(),
            _ => unimplemented!(),
        }
    }

    /// A local let binding: let x: u64 = s.parse()?.
    pub fn process_local(&mut self, local: &mut Local) {
        // first look at the attributes and parse the IFC related ones

        let ifc_attrs = self.local_attrs(local);
        // next we set the type type accordingly
        match &mut local.pat {
            Pat::Type(t) => self.process_pat_types(t, &ifc_attrs),
            Pat::Ident(ident) => self.process_pat_ident(ident, &ifc_attrs),
            _ => unimplemented!(),
        }

        // next we check that this is not low-var let statement with high expr condition guard
        if let Some(guard_span) = self.get_high_condition() {
            if VariableState::High != *ifc_attrs.state.get() {
                let local_span = local.span();
                let ident_span = self.get_lhs_span(local);
                high_guard(local_span, guard_span, ident_span).abort();
            }
        }
        // finally we set the initializer correctly.
        if let Some((_, expr)) = &mut local.init {
            // we have one of the following scenarios:
            //  1- expr is neither high not low, , in this case we can simply type it
            //     according to our ifc_attrs.
            //  2- expr is typed either high or low with the same type as in ifc_attr,
            //     in this case we do nothing
            //  3- expr is typed high and ifc_attr is low => Compiler Error
            //  4- expr is low and ifc_attr is high => we do casting.
            self.process_expr_with_attrs(expr, &ifc_attrs);
            let new_expr_tokens = match (self.get_expr_type(expr), ifc_attrs.state.get()) {
                // I got lost here for a moment and I thought I caught a bug,
                // but you really cannot declare new untyped vairable inside ifc block can you.
                (_, VariableState::None) => unreachable!(),
                (VariableState::None, VariableState::Low) => quote!(ifc::LowVar::new(#expr)).into(),
                (VariableState::None, VariableState::High) => {
                    quote!(ifc::HighVar::new(#expr)).into()
                }
                (VariableState::Low, VariableState::Low) => quote!(#expr).into(),
                (VariableState::Low, VariableState::High) => {
                    quote!(ifc::HighVar::from(#expr)).into()
                }
                (VariableState::High, VariableState::Low) => {
                    if *ifc_attrs.declassify.get() {
                        quote!((#expr).declassify()).into()
                    } else {
                        let expr_span = expr.span();
                        let local_span = local.span();
                        let ident_span = self.get_lhs_span(local);
                        assign_high2low(local_span, expr_span, ident_span).abort()
                    }
                }
                (VariableState::High, VariableState::High) => quote!(#expr).into(),
            };
            *expr = Box::new(parse::<Expr>(new_expr_tokens).expect(
                "Fatal Error: Ifc-macros had Quote generated rust code that failed to parse",
            ));
        }
    }
}
