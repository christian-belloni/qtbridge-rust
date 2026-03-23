// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use quote::quote;

use qt_gen_common::naming;
use syn::{punctuated::Punctuated, spanned::Spanned};
use crate::cpp_fun::CppFun;

// Struct that examines content of Rust function and does following:
// * finds cpp_fn! macro
// * extract C++ code from the scope of cpp_fn! macro and adds stores it to the collection of CppFun
// * replace cpp_fn! macro with the name of function containing given C++ code
pub struct CppFunProcessor {
    inline_fn_counter: usize,    // Counter of C++ functions inlined in Rust function
    inline_cpps: Vec<CppFun>,    // Collection of C++ functions inlined in Rust function
}

impl Default for CppFunProcessor {
    fn default() -> Self {
        Self::new()
    }
}

impl CppFunProcessor {
    pub fn new() -> Self {
        Self {
            inline_fn_counter: 0,
            inline_cpps: Vec::new(),
        }
    }

    pub fn get_inlined_cpp_funcs(&mut self) -> Vec<CppFun> {
        std::mem::take(&mut self.inline_cpps)
    }

    pub fn process(&mut self, src_block: &syn::Block) -> syn::Result<syn::Block> {
        let new_block = self.expand_block(src_block)?
            .unwrap_or_else(|| src_block.clone());
        Ok(new_block)
    }

    pub fn expand_block(&mut self, src: &syn::Block) -> syn::Result<Option<syn::Block>> {

        let src_stmts = &src.stmts;
        let maybe_new_stmts = src_stmts.iter()
            .map(|stmt| self.expand_stmt(stmt))
            .collect::<syn::Result<Vec<_>>>()?;

        if maybe_new_stmts.iter().any(Option::is_some) {
            let new_stmts = maybe_new_stmts.into_iter()
                .enumerate()
                .map(|(idx, stmt)| stmt.unwrap_or_else(|| src_stmts[idx].clone()))
                .collect();
            let new_block = syn::Block {
                stmts: new_stmts,
                ..src.clone()
            };
            return Ok(Some(new_block))
        }

        Ok(None)
    }

    pub fn expand_stmt(&mut self, src: &syn::Stmt) -> syn::Result<Option<syn::Stmt>> {
        let new_stmt = match src {
            syn::Stmt::Local(local) =>
                self.expand_local(local)?
                    .map(syn::Stmt::Local),

            syn::Stmt::Item(item) =>
                self.expand_item(item)?
                    .map(syn::Stmt::Item),

            syn::Stmt::Expr(expr, semi) =>
                self.expand_expr(expr)?
                    .map(|new_expr| syn::Stmt::Expr(new_expr, *semi)),

            _ => None,
        };

        Ok(new_stmt)
    }

    pub fn inline_function_cpp_name_for_num(num: usize) -> String {
        let prefix = naming::cpp::function::inline_function_prefix();
        format!("{prefix}_{num}")
    }

    // Handle binding like
    // let cpp_impl = cpp_fn!(|v: &QVariant, value: &mut String| -> bool {
    //     // C++ code goes here...
    // })
    // Substitute macro with name of generated function
    fn expand_local(&mut self, src: &syn::Local) -> syn::Result<Option<syn::Local>> {
        let Some(src_init) = src.init.as_ref() else {
            return Ok(None)
        };

        let new_init = self.expand_local_init(src_init)?;
        Ok(new_init.map(|init| syn::Local {
            init: Some(init),
            ..src.clone()
        }))
    }

    fn expand_local_init(&mut self, src: &syn::LocalInit) -> syn::Result<Option<syn::LocalInit>>{
        let mut new_init = None;

        if let syn::Expr::Macro(expr_macro) = src.expr.as_ref()
            && let Some(new_expr) = self.expand_expr_macro(expr_macro)? {
                new_init = Some(syn::LocalInit{
                    expr: Box::new(new_expr),
                    ..src.clone()
                })
            }

        if let Some((else_token, else_expr)) = &src.diverge
            && let syn::Expr::Macro(expr_macro) = else_expr.as_ref()
                && let Some(new_expr) = self.expand_expr_macro(expr_macro)? {
                    let mut init = new_init.unwrap_or_else(|| src.clone());
                    init.diverge = Some((*else_token, Box::new(new_expr)));
                    new_init = Some(init);
                }

        Ok(new_init)
    }

    fn expand_item(&mut self, item: &syn::Item) -> syn::Result<Option<syn::Item>> {
        let new_item: Option<syn::Item> = match item {
            syn::Item::Static(item_static) => {
                let new_expr = self.expand_expr(&item_static.expr)?;
                new_expr.map(|expr| syn::ItemStatic {
                    expr: Box::new(expr),
                    ..item_static.clone()
                }.into())
            },
            _ => None
        };
        Ok(new_item)
    }

    fn expand_expr(&mut self, expr: &syn::Expr) -> syn::Result<Option<syn::Expr>> {
        match expr {
            syn::Expr::Block(expr_block) => {
                if let Some(new_expr_block) = self.expand_expr_block(expr_block)? {
                    return Ok(Some(new_expr_block.into()))
                };
            },
            syn::Expr::Call(expr_call) => {
                if let Some(new_expr_call) = self.expand_expr_call(expr_call)? {
                    return Ok(Some(new_expr_call.into()))
                }
            },
            syn::Expr::Closure(expr_closure) => {
                if let Some(new_expr_closure) = self.expand_expr_closure(expr_closure)? {
                    return Ok(Some(new_expr_closure.into()))
                }
            },
            syn::Expr::Macro(expr_macro) => {
                if expr_macro.mac.path.is_ident("cpp_fn") {
                    return Err(syn::Error::new(expr_macro.span(), "Probably wrong usage of cpp_fn macro. Can't identify neither '=' nor '()' pattern."));
                }
            },
            _ => {},
        };

        Ok(None)
    }

    fn expand_expr_block(&mut self, src: &syn::ExprBlock) -> syn::Result<Option<syn::ExprBlock>> {
        let Some(new_block) = self.expand_block(&src.block)? else {
            return Ok(None);
        };

        let new_expr_block = syn::ExprBlock {
            block: new_block,
            ..src.clone()
        };
        Ok(Some(new_expr_block))
    }

    // Handle invoke of function returned from cpp_fn macro
    // E.g.
    //
    // cpp_fn!(|value: i32| -> bool {
    //     // C++ code goes here...
    // })(42);
    //
    // or traverse AST further down and handle potential occurrences of cpp_fn in nested entities
    fn expand_expr_call(&mut self, src: &syn::ExprCall) -> syn::Result<Option<syn::ExprCall>> {
        let src_func = src.func.as_ref();

        let mut new_func = None;
        if let syn::Expr::Macro(expr_macro) = src_func {
            new_func = self.expand_expr_macro(expr_macro)?;
        }

        // Check arguments of the call
        let expand_args = src.args.iter()
            .map(|arg_expr| self.expand_expr(arg_expr))
            .collect::<syn::Result<Vec<_>>>()?;

        let mut new_args = None;
        if expand_args.iter().any(Option::is_some) {
            let args: Vec<_> = expand_args.into_iter()
                .enumerate()
                .map(|(idx, arg_expr)| arg_expr.unwrap_or_else(|| src.args[idx].clone()))
                .collect();
            new_args = Some(Punctuated::from_iter(args));
        }

        if new_func.is_none() && new_args.is_none() {
            return Ok(None)
        }

        let new_expr_call = syn::ExprCall {
            func: Box::new(new_func.unwrap_or_else(|| src_func.clone())),
            args: new_args.unwrap_or_else(|| src.args.clone()),
            ..src.clone()
        };

        Ok(Some(new_expr_call))
    }

    fn expand_expr_closure(&mut self, src: &syn::ExprClosure) -> syn::Result<Option<syn::ExprClosure>> {
        let Some(new_body) = self.expand_expr(&src.body)? else {
            return Ok(None)
        };

        let new_expr_closure = syn::ExprClosure {
            body: Box::new(new_body),
            ..src.clone()
        };
        Ok(Some(new_expr_closure))
    }

    fn expand_expr_macro(&mut self, src: &syn::ExprMacro) -> syn::Result<Option<syn::Expr>>{
        if !src.mac.path.is_ident("cpp_fn") {
            return Ok(None)
        }

        let fn_name = self.inline_function_name();
        let cpp_fun = CppFun::new(fn_name, src.mac.tokens.clone())?;
        let fn_ident = cpp_fun.signature().ident.clone();

        self.inline_cpps.push(cpp_fun);

        let expr = syn::parse2::<syn::ExprPath>(quote!{
            ffi::#fn_ident
        })?;

        Ok(Some(syn::Expr::Path(expr)))

    }

    fn inline_function_name(&mut self) -> String {
        let num = &mut self.inline_fn_counter;
        let result = Self::inline_function_cpp_name_for_num(*num);
        *num += 1;
        result
    }
}
