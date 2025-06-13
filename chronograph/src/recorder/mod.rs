use std::fmt::Debug;

use crate::{recorder::batch::BatchingSpanRecorder, schema::SpanData};

pub mod batch;

/// Records spans, which can either be:
/// - a [BatchingSpanRecorder]
/// - a user-provided [RecordSpan] struct, which is called via dynamic dispatch
/// - a no-op recorder, which does nothing
pub enum SpanRecorder {
    Batching(BatchingSpanRecorder),
    Dyn(Box<dyn RecordSpan>),
    NoOp(),
}

impl Debug for SpanRecorder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Batching(_) => write!(f, "Batching"),
            Self::Dyn(_) => write!(f, "Dyn"),
            Self::NoOp() => write!(f, "NoOp"),
        }
    }
}

impl From<BatchingSpanRecorder> for SpanRecorder {
    fn from(value: BatchingSpanRecorder) -> Self {
        Self::Batching(value)
    }
}

impl From<Box<dyn RecordSpan>> for SpanRecorder {
    fn from(value: Box<dyn RecordSpan>) -> Self {
        Self::Dyn(value)
    }
}

impl From<()> for SpanRecorder {
    fn from(_: ()) -> Self {
        Self::NoOp()
    }
}

/// Used in [SpanRecorder::Dyn] to allow users to provide their own span recorder.
pub trait RecordSpan: Send + Sync {
    fn record_span(&self, span: SpanData);
}

impl<F: Fn(SpanData) + Send + Sync> RecordSpan for F {
    fn record_span(&self, span: SpanData) {
        self(span)
    }
}

impl SpanRecorder {
    pub fn record_span(&self, span: SpanData) {
        match self {
            Self::Batching(x) => x.record_span(span),
            Self::Dyn(x) => x.record_span(span),
            Self::NoOp() => {}
        }
    }
}
