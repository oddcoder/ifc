use crate::error::redundant_attr;
use proc_macro2::Span;
use std::cell::RefCell;
pub struct Property<T> {
    p: T,
    span: Option<Span>,
    accessed: RefCell<bool>,
}

impl<T> Property<T> {
    pub fn new(p: T) -> Property<T> {
        Property {
            p,
            span: None,
            accessed: RefCell::new(false),
        }
    }

    pub fn new_with_span(p: T, span: Span) -> Property<T> {
        Property {
            p,
            span: Some(span),
            accessed: RefCell::new(false),
        }
    }
    pub fn get(&self) -> &T {
        self.accessed.replace(true);
        &self.p
    }
    /*pub fn get_span(&self) -> Option<&Span> {
        self.span.as_ref()
    }*/
}

impl<T> Drop for Property<T> {
    fn drop(&mut self) {
        if let Some(span) = self.span {
            if !*self.accessed.borrow() {
                redundant_attr(span);
            }
        }
    }
}
