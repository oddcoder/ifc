mod property;

use crate::error::{conflicting_attributes, redundant_high, unknown_attribute};
use proc_macro2::Ident;
use proc_macro2::Span;
use property::Property;
#[derive(Debug, PartialEq)]
pub(crate) enum VariableState {
    /// None is for variables declared outside the scope of IFC marcros.
    None,
    Low,
    High,
}

#[derive(Default)]
struct AttributesBuilder {
    state: Option<VariableState>,
    state_span: Option<Span>,
    r#unsafe: Option<bool>,
    unsafe_span: Option<Span>,
}

impl AttributesBuilder {
    fn new() -> AttributesBuilder {
        Default::default()
    }
    fn set_state(&mut self, state: VariableState, span: Span) {
        match self.state.as_ref() {
            Some(_) => {
                if self.state_span.unwrap().start() > span.start() {
                    conflicting_attributes(span, self.state_span.unwrap()).abort();
                } else {
                    conflicting_attributes(self.state_span.unwrap(), span).abort();
                }
            }
            None => {
                self.state = Some(state);
                self.state_span = Some(span);
            }
        }
    }
    fn set_unsafe(&mut self, span: Span) {
        match self.r#unsafe.as_ref() {
            Some(_) => {
                if self.state_span.unwrap().start() > span.start() {
                    conflicting_attributes(span, self.state_span.unwrap()).abort();
                } else {
                    conflicting_attributes(self.state_span.unwrap(), span).abort();
                }
            }
            None => {
                self.r#unsafe = Some(true);
                self.unsafe_span = Some(span);
            }
        }
    }
    fn consume(&mut self, i: Ident) {
        let ident_str = format!("{}", i);
        match &*ident_str {
            "High" => {
                redundant_high(i.span());
                self.set_state(VariableState::High, i.span());
            }
            "Low" => self.set_state(VariableState::Low, i.span()),
            "Unsafe" => self.set_unsafe(i.span()),
            _ => unknown_attribute(i.span()).abort(),
        }
    }
}

impl From<AttributesBuilder> for Attributes {
    fn from(b: AttributesBuilder) -> Self {
        let state = match b.state {
            Some(s) => Property::new_with_span(s, b.state_span.unwrap()),
            None => Property::new(VariableState::High),
        };
        let r#unsafe = match b.r#unsafe {
            Some(s) => Property::new_with_span(s, b.unsafe_span.unwrap()),
            None => Property::new(false),
        };
        Attributes { state, r#unsafe }
    }
}

pub(crate) struct Attributes {
    pub(crate) state: Property<VariableState>,
    pub(crate) r#unsafe: Property<bool>,
}

impl Attributes {
    pub fn new(attrs: Vec<Ident>) -> Attributes {
        let mut builder = AttributesBuilder::new();
        for ident in attrs {
            builder.consume(ident);
        }
        builder.into()
    }
}
