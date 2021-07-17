mod attributes;
mod expr;
mod local;

use std::collections::{HashMap, HashSet};
use syn::{Ident, Stmt, Type};

#[derive(Default)]
pub struct IfcContext {
    high_vars: HashSet<Ident>,
    low_vars: HashSet<Ident>,
    objects: HashMap<Ident, Box<Type>>,
}

impl IfcContext {
    pub fn new() -> Self {
        Default::default()
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
