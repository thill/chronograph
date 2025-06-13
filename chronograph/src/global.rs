//! Global chronograph management.
//!
//! This module provides functions for getting and initializing the global chronograph.
//!
//! # Example
//! ```rust
//! use chronograph::{global, init, Chronograph};
//! let chronograph = Chronograph::builder().build();
//! init(chronograph);
//! let mut span = global().start_span();
//! span.record_instant("my_op_start");
//! span.record_value("count", 42);
//! span.record_instant("my_op_end");
//! ```

use crate::{recorder::SpanRecorder, Chronograph};
use std::sync::OnceLock;

static GLOBAL_CHRONOGRAPH: OnceLock<Chronograph> = OnceLock::new();
static NOOP_CHRONOGRAPH: OnceLock<Chronograph> = OnceLock::new();

/// Get a reference to the global chronograph.
/// If `init` has not been called, a no-op chronograph will be returned.
pub fn global() -> &'static Chronograph {
    fn get_noop_chronograph() -> &'static Chronograph {
        NOOP_CHRONOGRAPH.get_or_init(|| {
            Chronograph::builder()
                .with_recorder(SpanRecorder::NoOp())
                .build()
        })
    }
    GLOBAL_CHRONOGRAPH
        .get()
        .unwrap_or_else(get_noop_chronograph)
}

/// Initialize the global chronograph.
///
/// # Panics
///
/// This function will panic if it is called more than once.
pub fn init(chronograph: Chronograph) {
    if GLOBAL_CHRONOGRAPH.set(chronograph).is_err() {
        panic!("chronograph::init has already been called");
    }
}
