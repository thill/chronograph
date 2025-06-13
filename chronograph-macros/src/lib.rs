//! Macros for convenient chronograph span operations.
//!
//! These macros provide ergonomic ways to interact with chronograph's thread-local spans.
//! They wrap the thread-local span functionality to make it easier to use in your code.

use proc_macro::TokenStream;
use quote::quote;
use syn::parse::{Parse, ParseStream, Result};
use syn::{parse_macro_input, Expr, Token};

/// Start a new thread-local span using the global chronograph.
///
/// # Example
/// ```rust
/// start_span!();
/// ```
#[proc_macro]
pub fn start_span(_input: TokenStream) -> TokenStream {
    quote! {
        chronograph::start_threadlocal_span()
    }
    .into()
}

/// Record an instant datapoint in the current thread-local span.
///
/// # Example
/// ```rust
/// record_instant!("my_datapoint");
/// ```
#[proc_macro]
pub fn record_instant(input: TokenStream) -> TokenStream {
    let expr = parse_macro_input!(input as Expr);
    quote! {
        chronograph::get_threadlocal_span().record_instant(#expr)
    }
    .into()
}

/// Record a unix time datapoint in the current thread-local span.
///
/// # Example
/// ```rust
/// record_unix_time!("timestamp");
/// ```
#[proc_macro]
pub fn record_unix_time(input: TokenStream) -> TokenStream {
    let expr = parse_macro_input!(input as Expr);
    quote! {
        chronograph::get_threadlocal_span().record_unix_time(#expr)
    }
    .into()
}

struct ValueInput {
    id: Expr,
    _comma: Token![,],
    value: Expr,
}

impl Parse for ValueInput {
    fn parse(input: ParseStream) -> Result<Self> {
        Ok(ValueInput {
            id: input.parse()?,
            _comma: input.parse()?,
            value: input.parse()?,
        })
    }
}

/// Record a value datapoint in the current thread-local span.
///
/// # Example
/// ```rust
/// record_value!("count", 42);
/// ```
#[proc_macro]
pub fn record_value(input: TokenStream) -> TokenStream {
    let ValueInput { id, value, .. } = parse_macro_input!(input as ValueInput);
    quote! {
        chronograph::get_threadlocal_span().record_value(#id, #value)
    }
    .into()
}

/// Take the current thread-local span, leaving None in its place.
///
/// # Example
/// ```rust
/// let span = take_span!();
/// ```
#[proc_macro]
pub fn take_span(_input: TokenStream) -> TokenStream {
    quote! {
        chronograph::take_threadlocal_span()
    }
    .into()
}

/// End the current thread-local span, dropping it from memory.
///
/// # Example
/// ```rust
/// end_span!();
/// ```
#[proc_macro]
pub fn end_span(_input: TokenStream) -> TokenStream {
    quote! {
        chronograph::end_threadlocal_span()
    }
    .into()
}
