use super::IfcContext;
use crate::attributes::{Attributes, VariableState};
use crate::error::{assign_high2low, high_guard, high_guard_fn, pass_high_to_fn};
use proc_macro2::Span;
use quote::quote;
use syn::spanned::Spanned;
use syn::{parse, Block, Expr, ExprCall, ExprIf, ExprMatch, ExprWhile, Pat, Stmt};

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
            match (
                self.get_high_condition(),
                *attrs.r#unsafe.get(),
                self.get_expr_type(argument),
            ) {
                (Some(guard), false, _) => high_guard_fn(callspan, guard, argument.span()).abort(),
                (_, _, VariableState::None) => (),
                (_, _, VariableState::Low) => {
                    let tokens = quote!(#argument.inner());
                    *argument = parse::<Expr>(tokens.into()).expect(
                        "Fatal Error: Ifc-macros had Quote generated rust code that failed to parse",
                    );
                }
                (_, false, VariableState::High) => {
                    pass_high_to_fn(callspan, argument.span()).abort()
                }
                (_, true, VariableState::High) => {
                    let tokens = quote!(unsafe{#argument.inner()});
                    *argument = parse::<Expr>(tokens.into()).expect(
                        "Fatal Error: Ifc-macros had Quote generated rust code that failed to parse",    
                    );
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
        let tokens = match (
            self.get_high_condition(),
            self.get_expr_type(left),
            self.get_expr_type(right),
        ) {
            (Some(guard), VariableState::None, _) => {
                high_guard(fullspan, guard, left.span()).abort()
            }
            (Some(guard), VariableState::Low, _) => {
                high_guard(fullspan, guard, left.span()).abort()
            }
            (None, VariableState::None, VariableState::None) => quote!(#right),
            (None, VariableState::None, VariableState::Low) => quote!(#right.inner()),
            (None, VariableState::None, VariableState::High) => {
                if *attrs.declassify.get() {
                    quote!(#right.declassify().inner()).into()
                } else {
                    assign_high2low(fullspan, right.span(), left.span()).abort()
                }
            }
            (None, VariableState::Low, VariableState::None) => quote!(ifc::LowVar::new(#right)),
            (None, VariableState::Low, VariableState::Low) => quote!(#right),
            (None, VariableState::Low, VariableState::High) => {
                if *attrs.declassify.get() {
                    quote!(#right.declassify()).into()
                } else {
                    assign_high2low(fullspan, right.span(), left.span()).abort()
                }
            }
            (_, VariableState::High, VariableState::None) => quote!(ifc::HighVar::new(#right)),
            (_, VariableState::High, VariableState::Low) => {
                quote!(ifc::HighVar::<_>::from(#right))
            }
            (_, VariableState::High, VariableState::High) => quote!(#right),
        };
        *right = parse::<Expr>(tokens.into())
            .expect("Fatal Error: Ifc-macros had Quote generated rust code that failed to parse");
    }

    fn process_binary_sides(&mut self, left: &mut Expr, right: &mut Expr, attrs: &Attributes) {
        self.process_expr_with_attrs(left, attrs);
        self.process_expr_with_attrs(right, attrs);
        let left_type = self.get_expr_type(left);
        let right_type = self.get_expr_type(right);
        match (left_type, right_type) {
            (VariableState::None, VariableState::None) => (),
            (VariableState::None, VariableState::Low) => {
                let tokens = quote!(ifc::LowVar::new(#left));
                *left = parse::<Expr>(tokens.into()).expect(
                    "Fatal Error: Ifc-macros had Quote generated rust code that failed to parse",
                );
            }
            (VariableState::None, VariableState::High) => {
                let tokens = quote!(ifc::HighVar::new(#left));
                *left = parse::<Expr>(tokens.into()).expect(
                    "Fatal Error: Ifc-macros had Quote generated rust code that failed to parse",
                );
            }
            (VariableState::Low, VariableState::None) => {
                let tokens = quote!(ifc::LowVar::new(#right));
                *right = parse::<Expr>(tokens.into()).expect(
                    "Fatal Error: Ifc-macros had Quote generated rust code that failed to parse",
                );
            }
            (VariableState::Low, VariableState::Low) => (),
            (VariableState::Low, VariableState::High) => {
                let tokens = quote!(ifc::HighVar::<_>::from(#left));
                *left = parse::<Expr>(tokens.into()).expect(
                    "Fatal Error: Ifc-macros had Quote generated rust code that failed to parse",
                );
            }
            (VariableState::High, VariableState::None) => {
                let tokens = quote!(ifc::HighVar::new(#right));
                *right = parse::<Expr>(tokens.into()).expect(
                    "Fatal Error: Ifc-macros had Quote generated rust code that failed to parse",
                );
            }
            (VariableState::High, VariableState::Low) => {
                let tokens = quote!(ifc::HighVar::<_>::from(#right));
                *right = parse::<Expr>(tokens.into()).expect(
                    "Fatal Error: Ifc-macros had Quote generated rust code that failed to parse",
                );
            }
            (VariableState::High, VariableState::High) => (),
        }
    }
    fn process_block(&mut self, block: &mut Block, guard: Option<Span>, _: &Attributes) {
        self.add_scope(block.span());
        if let Some(guard) = guard {
            self.set_high_condition(guard);
        }
        for stmt in block.stmts.iter_mut() {
            self.process_stmt(stmt);
        }
        self.remove_scope();
    }
    fn process_if(&mut self, ifexpr: &mut ExprIf, attrs: &Attributes) {
        let span = ifexpr.cond.span();
        self.process_expr_with_attrs(&mut ifexpr.cond, attrs);
        let expr_type = self.get_expr_type(&ifexpr.cond);
        let guardspan = match expr_type {
            VariableState::High => Some(span),
            _ => None,
        };
        self.process_block(&mut ifexpr.then_branch, guardspan, attrs);

        if let Some((_, expr)) = ifexpr.else_branch.as_mut() {
            self.add_scope(expr.span());
            // XXX 2 scopes same span ... we are storing in hashmap
            // ... the problem is that expr is most likely block
            if let Some(span) = guardspan {
                self.set_high_condition(span)
            }
            self.process_expr_with_attrs(expr, attrs);
            self.remove_scope();
        }
    }
    fn process_while(&mut self, wexpr: &mut ExprWhile, attrs: &Attributes) {
        let span = wexpr.cond.span();
        self.process_expr_with_attrs(&mut wexpr.cond, attrs);
        let guardspan = match self.get_expr_type(&wexpr.cond) {
            VariableState::High => Some(span),
            _ => None,
        };
        self.process_block(&mut wexpr.body, guardspan, attrs);
    }
    fn process_match(&mut self, mexpr: &mut ExprMatch, attrs: &Attributes) {
        let span = mexpr.expr.span();
        self.process_expr_with_attrs(&mut mexpr.expr, attrs);
        let (guardspan, arm_transform) = match self.get_expr_type(&mexpr.expr) {
            VariableState::High => {
                let transform: Box<dyn Fn(&mut Pat) + 'static> = Box::new(|p: &mut Pat| {
                    let tokens = quote!(ifc::HighVar{data: #p, status: std::marker::PhantomData});
                    *p = parse::<Pat>(tokens.into()).expect(
                        "Fatal Error: Ifc-macros had Quote generated rust code that failed to parse",
                    );
                });
                (Some(span), transform)
            }
            VariableState::Low => {
                let transform: Box<dyn Fn(&mut Pat) + 'static> = Box::new(|p: &mut Pat| {
                    let tokens = quote!(ifc::LowVar{data: #p, status: std::marker::PhantomData});
                    *p = parse::<Pat>(tokens.into()).expect(
                        "Fatal Error: Ifc-macros had Quote generated rust code that failed to parse",
                    );
                });
                (None, transform)
            }
            VariableState::None => {
                let transform: Box<dyn Fn(&mut Pat) + 'static> = Box::new(|_p: &mut Pat| ());
                (None, transform)
            }
        };
        for arm in mexpr.arms.iter_mut() {
            arm_transform(&mut arm.pat);
            self.add_scope(arm.body.span());
            if let Some(span) = guardspan {
                self.set_high_condition(span)
            }
            self.process_expr_with_attrs(&mut arm.body, attrs);
            self.remove_scope();
        }
    }
    pub(crate) fn process_expr_with_attrs(&mut self, expr: &mut Expr, attrs: &Attributes) {
        match expr {
            Expr::Assign(assign) => {
                self.process_assign_sides(assign.span(), &mut assign.left, &mut assign.right, attrs)
            }
            Expr::AssignOp(assign) => {
                self.process_assign_sides(assign.span(), &mut assign.left, &mut assign.right, attrs)
            }
            Expr::Binary(b) => self.process_binary_sides(&mut b.left, &mut b.right, attrs),
            Expr::Block(b) => self.process_block(&mut b.block, None, attrs),
            Expr::Call(call) => self.process_call(call, attrs),
            Expr::If(ifexpr) => self.process_if(ifexpr, attrs),
            // we do not do any transoformations to literals.
            Expr::Lit(_) => (),
            Expr::Match(mexpr) => self.process_match(mexpr, attrs),
            Expr::Paren(paren) => self.process_expr_with_attrs(&mut paren.expr, attrs),
            // we don't do any transformations on identifiers.
            Expr::Path(_) => (),
            Expr::Reference(r) => self.process_expr_with_attrs(&mut r.expr, attrs),
            Expr::Unary(u) => self.process_expr_with_attrs(&mut u.expr, attrs),
            Expr::While(wexpr) => self.process_while(wexpr, attrs),
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

    fn get_block_type(&mut self, block: &Block) -> VariableState {
        // lets imagine the following let x = { //code here}
        // the value of is identified by the last statement on {}
        // but if the last statement is not expression (with out simicolong)
        // then the block returns `()` which is VariableState::None
        // It is important to note that VariableState::None means that statement
        // is neigher high nor low
        self.tmp_add_scope(block.span());
        let ty = match block.stmts.last() {
            Some(Stmt::Expr(e)) => self.get_expr_type(e),
            _ => VariableState::None,
        };
        self.tmp_remove_scope();
        ty
    }
    pub(crate) fn get_expr_type(&mut self, expr: &Expr) -> VariableState {
        match expr {
            // If an assignment is well typed
            // Then left and right have the same types
            Expr::Assign(assign) => self.get_expr_type(&assign.left),
            // If an assignment is well typed
            // Then left and right have the same types
            Expr::AssignOp(assign) => self.get_expr_type(&assign.left),
            Expr::Binary(b) => match (self.get_expr_type(&b.left), self.get_expr_type(&b.right)) {
                (VariableState::None, t) => t,
                (t, VariableState::None) => t,
                (VariableState::Low, t) => t,
                (VariableState::High, _) => VariableState::High,
            },
            Expr::Block(b) => self.get_block_type(&b.block),
            // we don't support functions yet so we treat them as untyped.
            Expr::Call(_) => VariableState::None,
            // If the condition of the If Expression is high then the return
            // value must be high (rust doesn't enforce it) If the condition
            // is low and the if expression is well typed then the type of
            //the then block should be the same as the type of the else block.
            Expr::If(ifexpr) => match self.get_expr_type(&ifexpr.cond) {
                VariableState::High => VariableState::High,
                _ => self.get_block_type(&ifexpr.then_branch),
            },
            // Literals are not typed
            // This way we can have them wrapped by High or low immediately
            Expr::Lit(_) => VariableState::None,
            Expr::Match(mexpr) => match self.get_expr_type(&mexpr.expr) {
                VariableState::High => VariableState::High,
                _ => self.get_expr_type(&mexpr.arms[0].body),
            },

            // parens are for (a + b) and if conditions and loops
            // Not to be confused with tuples
            Expr::Paren(paren) => self.get_expr_type(&paren.expr),
            Expr::Path(p) => {
                // we don't support High/Low variables from differnet modules.
                // if path is composed of more than one segments
                match p.path.get_ident() {
                    Some(ident) => self.get_type(ident),
                    // We don't support paths
                    None => VariableState::None,
                }
            }
            Expr::Reference(r) => self.get_expr_type(&r.expr),
            Expr::Unary(u) => self.get_expr_type(&u.expr),
            Expr::While(wexpr) => match self.get_expr_type(&wexpr.cond) {
                VariableState::High => VariableState::High,
                _ => self.get_block_type(&wexpr.body),
            },
            _ => {
                println!("get_expr_type {:#?}", expr);
                unimplemented!();
            } /*
              //consider arrays of high or low variables
                Expr::Array(_) => None,
                Expr::Async(_) unimplemented!(),
                Expr::Await(ExprAwait) => unimplemented!(),
                Expr::Box(ExprBox),
                Expr::Break(ExprBreak),
                Expr::Cast(ExprCast),
                Expr::Closure(ExprClosure),
                Expr::Continue(ExprContinue),
                Expr::Field(ExprField),
                Expr::ForLoop(ExprForLoop),
                Expr::Group(ExprGroup),
                Expr::Index(ExprIndex),
                Expr::Let(ExprLet),
                Expr::Loop(ExprLoop),
                Expr::Macro(ExprMacro),
                Expr::MethodCall(ExprMethodCall),
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
                Expr::Yield(ExprYield),
                */
        }
    }
}
