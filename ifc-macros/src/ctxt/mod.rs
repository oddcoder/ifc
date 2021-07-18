mod attributes;
mod expr;
mod local;

use std::collections::{HashMap, HashSet};
use syn::{Ident, Stmt, Type};
use crate::scope::Scope;
use crate::attributes::*;

#[derive(Default)]
pub struct IfcContext {
    high_vars: HashSet<Ident>,
    low_vars: HashSet<Ident>,
    objects: HashMap<Ident, Box<Type>>,
    scopes: Vec<Scope>
}

impl IfcContext {
    pub fn new() -> Self {
        IfcContext {
            high_vars: Default::default(),
            low_vars: Default::default(),
            objects: Default::default(),
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
}
