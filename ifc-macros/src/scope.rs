use crate::attributes::*;
use std::collections::HashSet;
use syn::{Ident, Type};

#[derive(Default)]
pub struct Scope {
    high_vars: HashSet<Ident>,
    low_vars: HashSet<Ident>,
}

impl Scope {
    pub(crate) fn new() {
        Default::default()
    }
    pub(crate) fn add_high(&mut self, ident: Ident) {
        self.high_vars.insert(ident);
    }
    pub(crate) fn add_low(&mut self, ident: Ident) {
        self.low_vars.insert(ident);
    }
    pub(crate) fn get_type(&self, ident: &Ident) -> VariableState {
        if self.high_vars.contains(ident) {
            VariableState::High
        } else if self.low_vars.contains(ident) {
            VariableState::Low
        } else {
            VariableState::None
        }
    }
}
