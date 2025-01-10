// comp: mapping for_if_clause
//
// mapping: expression
//
// for_if_clause:
//     | 'for' pattern 'in' expression ('if' expression)*
//
// pattern: name (, name)*

use proc_macro2::TokenStream as TokenStream2;
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::{parse_macro_input, Expr, Pat, Token};

struct Comp {
    mapping: Mapping,
    for_if_clause: ForIfClause,
}

impl Parse for Comp {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            mapping: input.parse()?,
            for_if_clause: input.parse()?,
        })
    }
}

impl quote::ToTokens for Comp {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        let Mapping(mapping) = &self.mapping;
        let ForIfClause {
            pattern,
            sequence,
            conditions,
        } = &self.for_if_clause;

        let conditions = conditions.iter().map(|c| {
            let inner = &c.0;
            quote! { #inner }
        });

        tokens.extend(quote! {
            core::iter::IntoIterator::into_iter(#sequence).filter_map(|#pattern| {
                (true #(&& (#conditions))*).then(|| #mapping)
            })
        });
    }
}

#[derive(Debug)]
struct Mapping(Expr);

impl Parse for Mapping {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self(input.parse()?))
    }
}

impl ToTokens for Mapping {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        self.0.to_tokens(tokens);
    }
}

struct ForIfClause {
    pattern: Pattern,
    sequence: Expr,
    conditions: Vec<Condition>,
}

impl Parse for ForIfClause {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let _: Token![for] = input.parse()?;
        let pattern: Pattern = input.parse()?;
        let _: Token![in] = input.parse()?;
        let sequence: Expr = input.parse()?;
        let conditions = parse_zero_or_more(input);

        Ok(Self {
            pattern,
            sequence,
            conditions,
        })
    }
}

fn parse_zero_or_more<T: Parse>(input: ParseStream) -> Vec<T> {
    let mut result = Vec::new();
    while let Ok(item) = input.parse() {
        result.push(item);
    }
    result
}

struct Pattern(Pat);

impl Parse for Pattern {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.call(Pat::parse_single).map(Self)
    }
}

impl ToTokens for Pattern {
    fn to_tokens(&self, tokens: &mut TokenStream2) {
        self.0.to_tokens(tokens);
    }
}

struct Condition(Expr);

impl Parse for Condition {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let _: Token![if] = input.parse()?;
        input.parse().map(Self)
    }
}

#[proc_macro]
pub fn comp(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let c = parse_macro_input!(input as Comp);
    quote! { #c }.into()
}
