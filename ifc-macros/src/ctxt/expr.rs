use super::IfcContext;
use crate::attributes::{Attributes, VariableState};
use crate::error::{assign_high2low, pass_high_to_fn};
use proc_macro2::Span;
use quote::quote;
use syn::spanned::Spanned;
use syn::{parse, Expr, ExprCall};

impl IfcContext {
    /// We don't support IFC in functions yet.
    /// so we have 3 situations here.
    /// 1- argument is neither High nor low: in this case we don nothing
    /// 2- argument is low so we access the internals
    /// 3- argument is high: we only access internals if unsafe is provided.
    fn process_call(&mut self, call: &mut ExprCall, attrs: &Attributes) {
        let callspan = call.span();
        for argument in call.args.iter_mut() {
            // we discard attributes here
            self.process_expr_with_attrs(argument, attrs);
            match self.get_expr_type(argument) {
                None => (),
                Some(VariableState::Low) => {
                    let tokens = quote!(#argument.inner());
                    *argument = parse::<Expr>(tokens.into()).expect(
                        "Fatal Error: Ifc-macros had Quote generated rust code that failed to parse",
                    );
                }
                Some(VariableState::High) => {
                    if *attrs.r#unsafe.get() {
                        let tokens = quote!(unsafe{#argument.inner()});
                        *argument = parse::<Expr>(tokens.into()).expect(
                            "Fatal Error: Ifc-macros had Quote generated rust code that failed to parse",    
                        );
                    } else {
                        pass_high_to_fn(callspan, argument.span()).abort();
                    }
                }
            }
        }
    }

    fn process_assign_sides(
        &mut self,
        fullspan: Span,
        left: &mut Expr,
        right: &mut Expr,
        attrs: &Attributes,
    ) {
        self.process_expr_with_attrs(left, attrs);
        self.process_expr_with_attrs(right, attrs);
        let tokens = match (self.get_expr_type(left), self.get_expr_type(right)) {
            (None, None) => quote!(#right),
            (None, Some(VariableState::Low)) => quote!(#right.inner()),
            (None, Some(VariableState::High)) => {
                assign_high2low(fullspan, right.span(), left.span()).abort()
            }
            (Some(VariableState::Low), None) => quote!(ifc::LowVar::new(#right)),
            (Some(VariableState::Low), Some(VariableState::Low)) => quote!(#right),
            (Some(VariableState::Low), Some(VariableState::High)) => {
                assign_high2low(fullspan, right.span(), left.span()).abort()
            }
            (Some(VariableState::High), None) => quote!(ifc::HighVar::new(#right)),
            (Some(VariableState::High), Some(VariableState::Low)) => {
                quote!(ifc::HighVar::<_>::from(#right))
            }
            (Some(VariableState::High), Some(VariableState::High)) => quote!(#right),
        };
        *right = parse::<Expr>(tokens.into())
            .expect("Fatal Error: Ifc-macros had Quote generated rust code that failed to parse");
    }

    pub(crate) fn process_expr_with_attrs(&mut self, expr: &mut Expr, attrs: &Attributes) {
        match expr {
            Expr::Assign(assign) => {
                self.process_assign_sides(assign.span(), &mut assign.left, &mut assign.right, attrs)
            }
            Expr::AssignOp(assign) => {
                self.process_assign_sides(assign.span(), &mut assign.left, &mut assign.right, attrs)
            }

            Expr::Call(call) => self.process_call(call, attrs),
            // we do not do any transoformations to literals.
            Expr::Lit(_) => (),
            // we don't do any transformations on identifiers.
            Expr::Path(_) => (),
            Expr::Reference(r) => self.process_expr_with_attrs(&mut r.expr, attrs),
            Expr::Unary(u) => self.process_expr_with_attrs(&mut u.expr, attrs),
            _ => {
                println!("process_expr_with_attrs: {:#?}", expr);
                unimplemented!()
            }
        }
    }

    pub(crate) fn process_expr(&mut self, expr: &mut Expr) {
        let attrs = self.expr_attrs(expr);
        self.process_expr_with_attrs(expr, &attrs)
    }

    pub(crate) fn get_expr_type(&mut self, expr: &Expr) -> Option<VariableState> {
        match expr {
            // If an assignment is well typed
            // Then left and right have the same types
            Expr::Assign(assign) => self.get_expr_type(&assign.left),
            // If an assignment is well typed
            // Then left and right have the same types
            Expr::AssignOp(assign) => self.get_expr_type(&assign.left),
            // we don't support functions yet so we treat them as untyped.
            Expr::Call(_) => None,
            // Literals are not typed
            // This way we can have them wrapped by High or low immediately
            Expr::Lit(_) => None,
            Expr::Path(p) => {
                // we don't support High/Low variables from differnet modules.
                // if path is composed of more than one segments
                match p.path.get_ident() {
                    Some(ident) => {
                        if self.high_vars.contains(&ident) {
                            Some(VariableState::High)
                        } else if self.low_vars.contains(&ident) {
                            Some(VariableState::Low)
                        } else {
                            None
                        }
                    }
                    None => None,
                }
            }
            Expr::Reference(r) => self.get_expr_type(&r.expr),
            Expr::Unary(u) => self.get_expr_type(&u.expr),
            _ => {
                println!("get_expr_type {:#?}", expr);
                unimplemented!();
            } /*
              //consider arrays of high or low variables
                Expr::Array(_) => None,
                Expr::Async(_) unimplemented!(),
                Expr::Await(ExprAwait) => unimplemented!(),
                Expr::Binary(ExprBinary),
                Expr::Block(ExprBlock),
                Expr::Box(ExprBox),
                Expr::Break(ExprBreak),
                Expr::Cast(ExprCast),
                Expr::Closure(ExprClosure),
                Expr::Continue(ExprContinue),
                Expr::Field(ExprField),
                Expr::ForLoop(ExprForLoop),
                Expr::Group(ExprGroup),
                Expr::If(ExprIf),
                Expr::Index(ExprIndex),
                Expr::Let(ExprLet),
                Expr::Loop(ExprLoop),
                Expr::Macro(ExprMacro),
                Expr::Match(ExprMatch),
                Expr::MethodCall(ExprMethodCall),
                Expr::Paren(ExprParen),
                Expr::Range(ExprRange),
                Expr::Reference(ExprReference),
                Expr::Repeat(ExprRepeat),
                Expr::Return(ExprReturn),
                Expr::Struct(ExprStruct),
                Expr::Try(ExprTry),
                Expr::TryBlock(ExprTryBlock),
                Expr::Tuple(ExprTuple),
                Expr::Type(ExprType),
                Expr::Unsafe(ExprUnsafe),
                Expr::Verbatim(TokenStream),
                Expr::While(ExprWhile),
                Expr::Yield(ExprYield),
                */
        }
    }
}
