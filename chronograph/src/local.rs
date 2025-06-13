//! Local span management for thread-local spans.
//!
//! This module provides functions for setting, accessing, and taking thread-local spans.
//!
//! # Example
//! ```rust
//! use chronograph::{end_threadlocal_span, get_threadlocal_span, start_threadlocal_span};
//!
//! start_threadlocal_span();
//! get_threadlocal_span().record_instant("my_op_start");
//! get_threadlocal_span().record_value("count", 42);
//! get_threadlocal_span().record_instant("my_op_end");
//! end_threadlocal_span();
//! ```

use crate::Span;
use std::cell::RefCell;

thread_local! {
    static CURRENT_SPAN: RefCell<Option<Span>> = RefCell::new(None);
}

/// Start a new current thread-local span from the global chronograph.
pub fn start_threadlocal_span() {
    set_threadlocal_span(super::global().start_span());
}

/// Set the current thread-local span.
pub fn set_threadlocal_span(span: Span) {
    CURRENT_SPAN.with(|s| {
        *s.borrow_mut() = Some(span);
    });
}

/// Get a mutable reference to the current thread-local span.
/// If no span exists, a new one will be automatically created using the global chronograph.
/// This ensures that a valid span is always available.
pub fn get_threadlocal_span() -> &'static mut Span {
    CURRENT_SPAN.with(|s| {
        let mut span_ref = s.borrow_mut();
        if span_ref.is_none() {
            *span_ref = Some(super::global().start_span());
        }
        // Safety: we just ensured the Option is Some, and we need a static lifetime
        // to return a reference from a thread local. This is safe because the thread local
        // storage ensures the data lives for the thread's lifetime.
        unsafe { std::mem::transmute(span_ref.as_mut().unwrap()) }
    })
}

/// Take the current thread-local span, leaving `None` in its place.
///
/// This is useful to pass a span to pass to a new thread, where you can call [set_span] to set it.
pub fn take_threadlocal_span() -> Option<Span> {
    CURRENT_SPAN.with(|s| s.borrow_mut().take())
}

/// Explicitly end the current thread-local span, dropping it from memory if it existed.
pub fn end_threadlocal_span() {
    CURRENT_SPAN.with(|s| s.borrow_mut().take());
}
