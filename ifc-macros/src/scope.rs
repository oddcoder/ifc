use crate::attributes::*;
use proc_macro2::Span;
use std::collections::HashSet;
use syn::Ident;

#[derive(Default)]
pub struct Scope {
    high_vars: HashSet<Ident>,
    low_vars: HashSet<Ident>,
    /// signify that this scope of part of loop or
    high_condition: Option<Span>,
}

impl Scope {
    pub(crate) fn new() -> Scope {
        Default::default()
    }
    pub(crate) fn set_high_condition(&mut self, span: Span) {
        self.high_condition = Some(span);
    }
    pub(crate) fn get_high_condition(&self) -> Option<Span> {
        self.high_condition
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
