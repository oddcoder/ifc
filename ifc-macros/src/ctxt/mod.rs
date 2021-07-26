mod attributes;
mod expr;
mod local;

use crate::attributes::*;
use crate::scope::Scope;
use proc_macro2::Span;
use syn::{Ident, Stmt};

#[derive(Default)]
pub struct IfcContext {
    scopes: Vec<Scope>,
}

impl IfcContext {
    pub fn new() -> Self {
        IfcContext {
            scopes: vec![Scope::new()],
        }
    }

    fn add_high(&mut self, ident: Ident) {
        self.scopes.last_mut().unwrap().add_high(ident);
    }

    fn add_low(&mut self, ident: Ident) {
        self.scopes.last_mut().unwrap().add_low(ident);
    }

    fn get_type(&mut self, ident: &Ident) -> VariableState {
        for scope in self.scopes.iter().rev() {
            let s = scope.get_type(ident);
            if s != VariableState::None {
                return s;
            }
        }
        VariableState::None
    }

    fn add_scope(&mut self) {
        self.scopes.push(Scope::new())
    }

    fn remove_scope(&mut self) {
        self.scopes.pop();
    }

    pub fn process_stmt(&mut self, stmt: &mut Stmt) {
        match stmt {
            Stmt::Local(l) => self.process_local(l),
            Stmt::Item(_) => unimplemented!(),
            Stmt::Expr(e) => self.process_expr(e),
            Stmt::Semi(e, _) => self.process_expr(e),
        }
    }
    pub fn set_high_condition(&mut self, span: Span) {
        self.scopes.last_mut().unwrap().set_high_condition(span)
    }

    pub fn get_high_condition(&self) -> Option<Span> {
        for scope in self.scopes.iter().rev() {
            let high_expr = scope.get_high_condition();
            if high_expr.is_some() {
                return high_expr;
            }
        }
        None
    }
}
