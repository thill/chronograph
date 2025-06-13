//! Traits for user to hook into completed spans by reference.

use crate::schema::SpanData;

pub enum SpanProcessor {
    Dyn(Box<dyn ProcessSpan>),
}

impl SpanProcessor {
    pub fn post_process_span(&self, span_data: &SpanData) {
        match self {
            Self::Dyn(x) => x.process_span(span_data),
        }
    }
}

pub trait ProcessSpan: Send + Sync {
    fn process_span(&self, span: &SpanData);
}
