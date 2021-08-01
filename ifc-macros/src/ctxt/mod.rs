mod attributes;
mod expr;
mod local;

use crate::attributes::*;
use crate::scope::Scope;
use proc_macro2::Span;
use std::collections::HashMap;
use syn::{Ident, Stmt};
#[derive(Default)]
pub struct IfcContext {
    active_scopes: Vec<Scope>,
    /// This is stupid .. ideally I would have  HashMap<Span, Scope>
    /// But then Spans are not hashable
    /// So I use the string representation of the span :\
    dead_scopes: HashMap<String, Scope>,
}

impl IfcContext {
    pub fn new(span: Span) -> Self {
        IfcContext {
            active_scopes: vec![Scope::new(span)],
            dead_scopes: HashMap::new(),
        }
    }

    fn add_high(&mut self, ident: Ident) {
        self.active_scopes.last_mut().unwrap().add_high(ident);
    }

    fn add_low(&mut self, ident: Ident) {
        self.active_scopes.last_mut().unwrap().add_low(ident);
    }

    fn get_type(&mut self, ident: &Ident) -> VariableState {
        for scope in self.active_scopes.iter().rev() {
            let s = scope.get_type(ident);
            if s != VariableState::None {
                return s;
            }
        }
        VariableState::None
    }

    fn add_scope(&mut self, span: Span) {
        let mut scope = Scope::new(span);
        scope.add_parent(self.active_scopes.last().unwrap().get_span());
        self.active_scopes.push(scope);
    }

    fn remove_scope(&mut self) {
        let scope = self.active_scopes.pop();
        if let Some(scope) = scope {
            let span = format!("{:?}", scope.get_span());
            self.dead_scopes.insert(span, scope);
        }
    }
    fn tmp_add_scope(&mut self, span: Span) -> bool {
        //XXX welcome to stupidity.
        let key = format!("{:?}", span);
        let scope = self.dead_scopes.get(&key);
        match scope {
            Some(s) => {
                //XXX welcome to stupidity 2.0 .
                // Spans are not only not hashable, but they dont implement Eq trait. :\
                let span1 = format!("{:?}", s.get_parent().unwrap());
                let span2 = format!("{:?}", self.active_scopes.last().unwrap().get_span());
                assert_eq!(span1, span2);
                self.active_scopes.push(s.clone());
                true
            }
            _ => false,
        }
    }
    fn tmp_remove_scope(&mut self) {
        self.active_scopes.pop();
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
        self.active_scopes
            .last_mut()
            .unwrap()
            .set_high_condition(span)
    }

    pub fn get_high_condition(&self) -> Option<Span> {
        for scope in self.active_scopes.iter().rev() {
            let high_expr = scope.get_high_condition();
            if high_expr.is_some() {
                return high_expr;
            }
        }
        None
    }
}
