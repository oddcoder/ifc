use proc_macro2::Span;
use proc_macro_error::{Diagnostic, Level};

pub fn conflicting_attributes(span1: Span, span2: Span) -> Diagnostic {
    Diagnostic::spanned(
        span2,
        Level::Error,
        "Conflicting IFC attributes".to_string(),
    )
    .span_help(span1, "First attribute is here".to_string())
}

pub fn unknown_attribute(span: Span) -> Diagnostic {
    Diagnostic::spanned(span, Level::Error, "Unknown attribute".to_string())
}

pub fn redundant_high(span: Span) {
    Diagnostic::spanned(
        span,
        Level::Warning,
        "All variables are `High` by default".to_string(),
    )
    .help("Try removing `High`".to_string())
    .emit()
}

pub fn assign_high2low(full: Span, high: Span, low: Span) -> Diagnostic {
    Diagnostic::spanned(
        full,
        Level::Error,
        "Cannot assign high expression to low variable".to_string(),
    )
    .span_help(low, "Low Variable".to_string())
    .span_help(high, "High Expression".to_string())
}

pub fn redundant_attr(span: Span) {
    Diagnostic::spanned(span, Level::Warning, "Unused attribute".to_string())
        .help("Try removing it".to_string())
        .emit()
}

pub fn pass_high_to_fn(full: Span, high: Span) -> Diagnostic {
    Diagnostic::spanned(
        full,
        Level::Error,
        "Cannot pass high expression as an argument to functions or macros".to_string(),
    )
    .span_help(high, "High Expression".to_string())
}

pub fn high_guard(full: Span, high: Span, low: Span) -> Diagnostic {
    Diagnostic::spanned(
        full,
        Level::Error,
        "Cannot declare new low variable or assign to low variable in high context".to_string(),
    )
    .span_help(high, "High context was created here".to_string())
    .span_help(low, "Low variable is used here.".to_string())
}

pub fn high_guard_fn(full: Span, high: Span, arg: Span) -> Diagnostic {
    Diagnostic::spanned(
        full,
        Level::Error,
        "Cannot pass arguments to function that is called in high context".to_string(),
    )
    .span_help(high, "High context was created here".to_string())
    .span_help(arg, "Argument is passed here.".to_string())
}

pub fn macros_fail(mac: Span, args: Span) -> Diagnostic {
    Diagnostic::spanned(
        mac,
        Level::Error,
        "Cannot call macros that take arguments that are not function-like parameters.".to_string(),
    )
    .span_help(args, "Argument(s) is passed here.".to_string())
}
