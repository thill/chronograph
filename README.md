# chronograph

## Overview

A tracing library that allows you to efficiently record timestamps and metadata as datapoints within a span.

The `Chronograph` is the main entry point for starting spans.

Spans are recorded when they are dropped from memory where an underlying `SpanRecorder` is responsible for recording a span's data.

## Spans

All spans contain a unique monotonically increasing ID, a start unix time, a start instant, an end instant, and user datapoints.

- The start unix time is the unix time at the start of the span.
- The start instant is a monotonic instant, accurate nanosecond timer elapsed from when the Cronograph was started.
- The start instant can be used to calculate the duration of the span.
- The end instant is a monotonic instant, accurate nanosecond timer elapsed from when the Cronograph was started.
- User datapoints are typically recorded as "instant" time measurements, but they can also include metadata as simple types.

## Datapoints

User datapoints are recorded as a set of key-value pairs.

The `DatapointId` key is serialized as a u64 value.

- Any `impl Into<DatapointId>` can be used as a datapoint.
- A `u64` will be used as-is.
- A `&str` will be converted to a u64 using `zwohash` to derive a one-way, consistent u64 value.

The `RecordValue` can be one of the following types:

- Instant: A monotonic instant, accurate nanosecond timer elapsed from when the Cronograph was started.
- UnixTime: A unix time, as nanoseconds since epoch.
- Utf8String: A string value formatted as UTF-8.
- I32: A 32-bit signed integer.
- I64: A 64-bit signed integer.
- I128: A 128-bit signed integer.
- U32: A 32-bit unsigned integer.
- U64: A 64-bit unsigned integer.
- U128: A 128-bit unsigned integer.
- F32: A 32-bit floating point number.
- F64: A 64-bit floating point number.

## Sampling

Spans can elect to be sampled. It is most efficient to use a sampling rate that is a power of two.

## Global Chronograph

The global chronograph is a singleton that can be used to record spans.

It is initialized by calling the `init` function, and can be accessed with the `global` function.

## Macros

The `macros` module provides macros for recording datapoints.

- `macros::start_span` can be used to start a new thread-local span from the global chronograph.
- `macros::record_instant` can be used to record an instant datapoint to the current thread-local span.
- `macros::record_unix_time` can be used to record a unix time datapoint.
- `macros::record_value` can be used to record a value datapoint.
- `macros::end_span` can be used to end the current thread-local span.
- `macros::take_span` can be used to take the current thread-local span.

## Thread-local Spans

Thread-local spans can be used to record spans without needing to keep a reference to the `Span`.
These are the same thread-local spans used by the `macros` module.

A thread-local span can be started with the `global` chronograph by calling the `start_threadlocal_span` function.
You may alternative set it to any arbitraty span using the `set_threadlocal_span` function.

The thread-local span can be accessed with the `get_threadlocal_span` function or with the included `macros`.

`end_threadlocal_span` and `take_threadlocal_span` can be used to end/take the current thread-local span

## Global Instance Example with Macros

```rust
use std::time::Duration;
use std::thread::sleep;

use chronograph::macros::*;
use chronograph::recorder::batch::*;
use chronograph::schema::*;
use chronograph::Chronograph;

// Create a batching span recorder that will print each batch.
// Your code should elect to store the results somewhere.
// The collect callback function is called from a dedicated collector thread.
let recorder = BatchingSpanRecorder::start(
    Box::new(|batch: SpanBatch| {
        let serialized: Vec<u8> = batch.into();
        let deserialized = SpanBatch::try_from(serialized.as_slice()).unwrap();
        println!("collected {:?}", deserialized);
        println!("serialized to {} bytes", serialized.len());
    }),
    BatchCollectionOptions::default().with_batch_size_threshold(4),
);

// Initialize the global chronograph and record some random spans.
chronograph::init(
    Chronograph::builder()
        .with_recorder(recorder)
        .with_sample_rate(16) // it's most efficient to sample at a power of two
        .build(),
);

// Record some random data
for _ in 0..1000 {
    start_span!();
    record_instant!("my_op_start");
    record_value!("count", 42);
    record_instant!("mt_op_end");
    end_span!();
    sleep(Duration::from_millis(1));
}

// Give the collector thread a chance to wakeup and batch spans.
sleep(Duration::from_millis(100));
```

## Zero-Magic Example

Example without macros or global instances.

```rust
use chronograph::recorder::batch::*;
use chronograph::schema::*;
use chronograph::Chronograph;
use std::thread::sleep;
use std::time::Duration;

// Create a batching span recorder that will print each batch.
// Your code should elect to store the results somewhere.
// The collect callback function is called from a dedicated collector thread.
let recorder = BatchingSpanRecorder::start(
    Box::new(|batch: SpanBatch| {
        let serialized: Vec<u8> = batch.into();
        let deserialized = SpanBatch::try_from(serialized.as_slice()).unwrap();
        println!("collected {:?}", deserialized);
        println!("serialized to {} bytes", serialized.len());
    }),
    BatchCollectionOptions::default().with_batch_size_threshold(4),
);

// Create a chronograph and record some random spans.
let chronograph = Chronograph::builder()
    .with_recorder(recorder)
    .with_sample_rate(16) // it's most efficient to sample at a power of two
    .build();

// Record some random data
for _ in 0..1000 {
    let mut span = chronograph.start_span();
    span.record_instant("my_op_start");
    span.record_value("count", 42);
    span.record_instant("my_op_end");
    sleep(Duration::from_millis(1));
}

// Give the collector thread a chance to wakeup and batch spans.
sleep(Duration::from_millis(100));
```
