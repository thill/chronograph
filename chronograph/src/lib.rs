//! # Overview
//!
//! A tracing library that allows you to efficiently record timestamps and metadata as datapoints within a span.
//!
//! The [Chronograph] is the main entry point for starting spans.
//!
//! Spans are recorded when they are dropped from memory where an underlying [SpanRecorder] is responsible for recording a span's data.
//!
//!
//! # Spans
//!
//! All spans contain a unique monotonically increasing ID, a start unix time, a start instant, an end instant, and user datapoints.
//! - The start unix time is the unix time at the start of the span.
//! - The start instant is a monotonic instant, accurate nanosecond timer elapsed from when the Cronograph was started.
//! - The start instant can be used to calculate the duration of the span.
//! - The end instant is a monotonic instant, accurate nanosecond timer elapsed from when the Cronograph was started.
//! - User datapoints are typically recorded as "instant" time measurements, but they can also include metadata as simple types.
//!
//!
//! # Datapoints
//!
//! User datapoints are recorded as a set of key-value pairs.
//!
//! The [DatapointId] key is serialized as a u64 value.
//! - Any `impl Into<DatapointId>` can be used as a datapoint.
//! - A `u64` will be used as-is.
//! - A `&str` will be converted to a u64 using [zwohash] to derive a one-way, consistent u64 value.
//!
//! The [RecordValue] can be one of the following types:
//! - Instant: A monotonic instant, accurate nanosecond timer elapsed from when the Cronograph was started.
//! - UnixTime: A unix time, as nanoseconds since epoch.
//! - Utf8String: A string value formatted as UTF-8.
//! - I32: A 32-bit signed integer.
//! - I64: A 64-bit signed integer.
//! - I128: A 128-bit signed integer.
//! - U32: A 32-bit unsigned integer.
//! - U64: A 64-bit unsigned integer.
//! - U128: A 128-bit unsigned integer.
//! - F32: A 32-bit floating point number.
//! - F64: A 64-bit floating point number.
//!
//!
//! # Sampling
//!
//! Spans can elect to be sampled. It is most efficient to use a sampling rate that is a power of two.
//!
//!
//! # Global Chronograph
//!
//! The global chronograph is a singleton that can be used to record spans.
//!
//! It is initialized by calling the [init] function, and can be accessed with the [global] function.
//!
//!
//! # Macros
//!
//! The [macros] module provides macros for recording datapoints.
//! - [macros::start_span] can be used to start a new thread-local span from the global chronograph.
//! - [macros::record_instant] can be used to record an instant datapoint to the current thread-local span.
//! - [macros::record_unix_time] can be used to record a unix time datapoint.
//! - [macros::record_value] can be used to record a value datapoint.
//! - [macros::end_span] can be used to end the current thread-local span.
//! - [macros::take_span] can be used to take the current thread-local span.
//!
//!
//! # Thread-local Spans
//!
//! Thread-local spans can be used to record spans without needing to keep a reference to the [Span].
//! These are the same thread-local spans used by the [macros] module.
//!
//! A thread-local span can be started with the [global] chronograph by calling the [start_threadlocal_span] function.
//! You may alternative set it to any arbitraty span using the [set_threadlocal_span] function.
//!
//! The thread-local span can be accessed with the [get_threadlocal_span] function or with the included [macros].
//!
//! [end_threadlocal_span] and [take_threadlocal_span] can be used to end/take the current thread-local span
//!
//!
//! # Global Instance Example with Macros
//!
//! ```rust,no_run
//! use std::time::Duration;
//! use std::thread::sleep;
//!
//! use chronograph::macros::*;
//! use chronograph::recorder::batch::*;
//! use chronograph::schema::*;
//! use chronograph::Chronograph;
//!
//! // Create a batching span recorder that will print each batch.
//! // Your code should elect to store the results somewhere.
//! // The collect callback function is called from a dedicated collector thread.
//! let recorder = BatchingSpanRecorder::start(
//!     Box::new(|batch: SpanBatch| {
//!         let serialized: Vec<u8> = batch.into();
//!         let deserialized = SpanBatch::try_from(serialized.as_slice()).unwrap();
//!         println!("collected {:?}", deserialized);
//!         println!("serialized to {} bytes", serialized.len());
//!     }),
//!     BatchCollectionOptions::default().with_batch_size_threshold(4),
//! );
//!
//! // Initialize the global chronograph and record some random spans.
//! chronograph::init(
//!     Chronograph::builder()
//!         .with_recorder(recorder)
//!         .with_sample_rate(16) // it's most efficient to sample at a power of two
//!         .build(),
//! );
//!
//! // Record some random data
//! for _ in 0..1000 {
//!     start_span!();
//!     record_instant!("my_op_start");
//!     record_value!("count", 42);
//!     record_instant!("mt_op_end");
//!     end_span!();
//!     sleep(Duration::from_millis(1));
//! }
//!
//! // Give the collector thread a chance to wakeup and batch spans.
//! sleep(Duration::from_millis(100));
//! ```
//!
//!
//! # Zero-Magic Example
//!
//! Example without macros or global instances.
//!
//! ```rust,no_run
//! use chronograph::recorder::batch::*;
//! use chronograph::schema::*;
//! use chronograph::Chronograph;
//! use std::thread::sleep;
//! use std::time::Duration;
//!
//! // Create a batching span recorder that will print each batch.
//! // Your code should elect to store the results somewhere.
//! // The collect callback function is called from a dedicated collector thread.
//! let recorder = BatchingSpanRecorder::start(
//!     Box::new(|batch: SpanBatch| {
//!         let serialized: Vec<u8> = batch.into();
//!         let deserialized = SpanBatch::try_from(serialized.as_slice()).unwrap();
//!         println!("collected {:?}", deserialized);
//!         println!("serialized to {} bytes", serialized.len());
//!     }),
//!     BatchCollectionOptions::default().with_batch_size_threshold(4),
//! );
//!
//! // Create a chronograph and record some random spans.
//! let chronograph = Chronograph::builder()
//!     .with_recorder(recorder)
//!     .with_sample_rate(16) // it's most efficient to sample at a power of two
//!     .build();
//!
//! // Record some random data
//! for _ in 0..1000 {
//!     let mut span = chronograph.start_span();
//!     span.record_instant("my_op_start");
//!     span.record_value("count", 42);
//!     span.record_instant("my_op_end");
//!     sleep(Duration::from_millis(1));
//! }
//!
//! // Give the collector thread a chance to wakeup and batch spans.
//! sleep(Duration::from_millis(100));
//! ```

use std::{
    fmt::Debug,
    mem::take,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
    time::{Instant, SystemTime},
};

use crate::{
    processor::SpanProcessor,
    recorder::SpanRecorder,
    schema::{DatapointId, RecordData, RecordValue, SpanData},
};

pub mod processor;
pub mod recorder;
pub mod schema;

mod global;
mod local;

pub use global::{global, init};
pub use local::{
    end_threadlocal_span, get_threadlocal_span, set_threadlocal_span, start_threadlocal_span,
    take_threadlocal_span,
};

/// Re-export chronograph-macros as the macros module
pub use chronograph_macros as macros;

/// The main chronograph struct, which is used to start spans.
#[derive(Debug)]
pub struct Chronograph {
    context: Arc<ChronographContext>,
    next_id: AtomicU64,
    global_start_instant: Instant,
}

impl Chronograph {
    /// Start to build a [Chronograph] instance.
    pub fn builder() -> ChronographBuilder {
        ChronographBuilder {
            context: ChronographContext {
                processors: Vec::new(),
                recorder: SpanRecorder::NoOp(),
                sample_rate: SampleRate::All,
            },
        }
    }

    /// Start a new span. It will be recorded when it's dropped from memory.
    pub fn start_span(&self) -> Span {
        let span_id = self.next_id.fetch_add(1, Ordering::Relaxed);
        Span {
            sampled: self.context.sample_rate.sample(span_id),
            global_start_instant: self.global_start_instant,
            context: Arc::clone(&self.context),
            span_id,
            start_unix_time: SystemTime::now(),
            start_instant: self.global_start_instant.elapsed().as_nanos() as u64,
            records: Vec::new(),
        }
    }
}

/// Created using [Chronograph::builder]
#[derive(Debug)]
pub struct ChronographBuilder {
    context: ChronographContext,
}

impl ChronographBuilder {
    /// Set the single recorder, which takes ownership of the span data after processors are complete
    pub fn with_recorder(mut self, recorder: impl Into<SpanRecorder>) -> Self {
        self.context.recorder = recorder.into();
        self
    }

    /// Add a span processor, which are able to hook into span data by reference as it is finalized, before being recorded
    pub fn with_processor(mut self, post_processor: SpanProcessor) -> Self {
        self.context.processors.push(post_processor);
        self
    }

    pub fn with_sample_rate(mut self, sample_rate: u64) -> Self {
        self.context.sample_rate = SampleRate::from(sample_rate);
        self
    }

    /// Build the [Chronograph]
    pub fn build(self) -> Chronograph {
        Chronograph {
            context: Arc::new(self.context),
            next_id: AtomicU64::new(0),
            global_start_instant: Instant::now(),
        }
    }
}

/// A span records a set of related datapoints. The span data is recorded when the span is dropped from memory.
#[derive(Debug, Clone)]
pub struct Span {
    sampled: bool,
    global_start_instant: Instant,
    context: Arc<ChronographContext>,
    span_id: u64,
    start_unix_time: SystemTime,
    start_instant: u64,
    records: Vec<RecordData>,
}

impl Span {
    pub fn record_instant(&mut self, datapoint_id: impl Into<DatapointId>) -> &mut Self {
        if self.sampled {
            self.record_value(
                datapoint_id,
                RecordValue::Instant(self.global_start_instant.elapsed().as_nanos() as u64),
            );
        };
        self
    }

    pub fn record_unix_time(&mut self, datapoint_id: impl Into<DatapointId>) -> &mut Self {
        if self.sampled {
            self.record_value_no_sampling(
                datapoint_id,
                RecordValue::UnixTime(
                    self.start_unix_time
                        .duration_since(SystemTime::UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_nanos() as i64,
                ),
            );
        }
        self
    }

    pub fn record_value(
        &mut self,
        datapoint_id: impl Into<DatapointId>,
        value: impl Into<RecordValue>,
    ) -> &mut Self {
        if self.sampled {
            self.record_value_no_sampling(datapoint_id, value);
        }
        self
    }

    fn record_value_no_sampling(
        &mut self,
        datapoint_id: impl Into<DatapointId>,
        value: impl Into<RecordValue>,
    ) {
        self.records.push(RecordData {
            datapoint_id: datapoint_id.into(),
            value: value.into(),
        });
    }
}

impl Drop for Span {
    fn drop(&mut self) {
        if !self.sampled {
            return;
        }
        let span_data = SpanData {
            span_id: self.span_id,
            start_unix_time: self
                .start_unix_time
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap_or_default()
                .as_nanos() as i64,
            start_instant: self.start_instant,
            end_instant: self.global_start_instant.elapsed().as_nanos() as u64,
            records: take(&mut self.records),
        };
        for post_processor in self.context.processors.iter() {
            post_processor.post_process_span(&span_data);
        }
        self.context.recorder.record_span(span_data);
    }
}

struct ChronographContext {
    recorder: SpanRecorder,
    processors: Vec<SpanProcessor>,
    sample_rate: SampleRate,
}

impl Debug for ChronographContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ChronographContext")
            .field("recorder", &self.recorder)
            .field("sample_rate", &self.sample_rate)
            .field("processors_count", &self.processors.len())
            .finish()
    }
}

#[derive(Debug)]
enum SampleRate {
    All,
    Pow2(u64),
    Modulo(u64),
}

impl From<u64> for SampleRate {
    fn from(value: u64) -> Self {
        if value == 0 {
            Self::All
        } else if value.is_power_of_two() {
            Self::Pow2(value)
        } else {
            Self::Modulo(value)
        }
    }
}

impl SampleRate {
    pub fn sample(&self, span_id: u64) -> bool {
        match self {
            Self::All => true,
            Self::Pow2(x) => span_id & (x - 1) == 0,
            Self::Modulo(x) => span_id % x == 0,
        }
    }
}
