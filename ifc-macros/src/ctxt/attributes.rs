use super::IfcContext;

use crate::attributes::*;
use crate::error::conflicting_attributes;
use proc_macro2::{Ident, TokenTree};
use std::collections::HashSet;
use std::mem;
use syn::{Attribute, Expr, Local};

fn collect_ifc_tokens(attr: Attribute) -> Vec<Ident> {
    let mut tokens = Vec::new();
    for tree in attr.tokens {
        let stream = match tree {
            TokenTree::Group(g) => g.stream(),
            t => panic!("Expected #[IFC(Stuff)].. found `{}`", t),
        };
        for tree in stream {
            match tree {
                TokenTree::Ident(ident) => tokens.push(ident),
                TokenTree::Punct(p) => {
                    if p.as_char() != ',' {
                        panic!("Expected `,`, found {:#?}", p)
                    }
                }
                t => panic!("Expected Ident, found {:#?}", t),
            }
        }
    }
    tokens
}

macro_rules! extract_attributes {
    ($self: ident, $x: ident) => {{
        let attrs = $self.split_attrs(mem::take(&mut $x.attrs));
        $x.attrs = attrs.1;
        let ifc_attrs = attrs.0;
        Attributes::new(ifc_attrs)
    }};
}

impl IfcContext {
    /// Split Attributes based on if they are IFC or not
    fn split_attrs<I: IntoIterator<Item = Attribute>>(
        &self,
        attrs: I,
    ) -> (Vec<Ident>, Vec<Attribute>) {
        let mut ifc: HashSet<Ident> = HashSet::new();
        let mut non_ifc = Vec::new();
        for attr in attrs {
            if attr.path.is_ident("IFC") {
                let tokens = collect_ifc_tokens(attr);
                for token in tokens {
                    if ifc.contains(&token) {
                        let fst = ifc.get(&token).unwrap();
                        conflicting_attributes(fst.span(), token.span()).abort();
                    } else {
                        ifc.insert(token);
                    }
                }
            } else {
                non_ifc.push(attr);
            }
        }
        (ifc.into_iter().collect(), non_ifc)
    }

    /// process attributes for syn::Local
    pub(super) fn local_attrs(&self, local: &mut Local) -> Attributes {
        extract_attributes!(self, local)
    }

    /// process attributes for syn::Expr
    pub(super) fn expr_attrs(&self, expr: &mut Expr) -> Attributes {
        match expr {
            Expr::Array(a) => extract_attributes!(self, a),
            Expr::Assign(a) => extract_attributes!(self, a),
            Expr::AssignOp(a) => extract_attributes!(self, a),
            Expr::Async(a) => extract_attributes!(self, a),
            Expr::Await(a) => extract_attributes!(self, a),
            Expr::Binary(b) => extract_attributes!(self, b),
            Expr::Block(b) => extract_attributes!(self, b),
            Expr::Box(b) => extract_attributes!(self, b),
            Expr::Break(b) => extract_attributes!(self, b),
            Expr::Call(c) => extract_attributes!(self, c),
            Expr::Cast(c) => extract_attributes!(self, c),
            Expr::Closure(c) => extract_attributes!(self, c),
            Expr::Continue(c) => extract_attributes!(self, c),
            Expr::Field(f) => extract_attributes!(self, f),
            Expr::ForLoop(f) => extract_attributes!(self, f),
            Expr::Group(g) => extract_attributes!(self, g),
            Expr::If(i) => extract_attributes!(self, i),
            Expr::Index(i) => extract_attributes!(self, i),
            Expr::Let(l) => extract_attributes!(self, l),
            Expr::Lit(l) => extract_attributes!(self, l),
            Expr::Loop(l) => extract_attributes!(self, l),
            Expr::Macro(m) => extract_attributes!(self, m),
            Expr::Match(m) => extract_attributes!(self, m),
            Expr::MethodCall(m) => extract_attributes!(self, m),
            Expr::Paren(p) => extract_attributes!(self, p),
            Expr::Path(p) => extract_attributes!(self, p),
            Expr::Range(r) => extract_attributes!(self, r),
            Expr::Reference(r) => extract_attributes!(self, r),
            Expr::Repeat(r) => extract_attributes!(self, r),
            Expr::Return(r) => extract_attributes!(self, r),
            Expr::Struct(t) => extract_attributes!(self, t),
            Expr::Try(t) => extract_attributes!(self, t),
            Expr::TryBlock(t) => extract_attributes!(self, t),
            Expr::Tuple(t) => extract_attributes!(self, t),
            Expr::Type(t) => extract_attributes!(self, t),
            Expr::Unary(u) => extract_attributes!(self, u),
            Expr::Unsafe(u) => extract_attributes!(self, u),
            Expr::Verbatim(_) => Attributes::new(vec![]),
            Expr::While(w) => extract_attributes!(self, w),
            Expr::Yield(y) => extract_attributes!(self, y),
            // there is nothing left but there is undocumented thing called Expr::__TestExhaustive
            _ => unimplemented!(),
        }
    }
}
