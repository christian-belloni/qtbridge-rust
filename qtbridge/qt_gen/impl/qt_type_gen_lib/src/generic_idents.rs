// Copyright (C) 2025 The Qt Company Ltd.
// SPDX-License-Identifier: LicenseRef-Qt-Commercial OR LGPL-3.0-only

use syn::parse::Parse;
use syn::Token;

#[derive(Clone, Default)]
pub struct GenericIdents {
    list: Vec<syn::Ident>,
}

impl GenericIdents {
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn len(&self) -> usize {
        self.list.len()
    }

    pub fn list(&self) -> &[syn::Ident] {
        &self.list
    }
}

impl Parse for GenericIdents {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut list = Vec::new();

        let _left_angle: Token![<] = input.parse()?;
        loop {
            list.push(input.parse()?);
            if input.peek(Token![,]) {
                let _comma: Token![,] = input.parse()?;
                if input.peek(Token![>]) {
                    break;
                }
            }
            else if input.peek(Token![>]) {
                break;
            }
            else {
                return Err(input.error("Unexpected format of generic idents"));
            }
        }
        let _right_angle: Token![>] = input.parse()?;

        Ok(Self {
            list
        })
    }
}
