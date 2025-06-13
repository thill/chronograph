use std::{
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc,
    },
    time::{Duration, SystemTime},
};

use scc::Queue;

use crate::schema::{SpanBatch, SpanData};

/// A [super::SpanRecorder] that batches spans and sends them to a collector running in a separate thread
#[derive(Debug)]
pub struct BatchingSpanRecorder {
    batch: Arc<Queue<SpanData>>,
    batch_size_threshold: usize,
    thread_tx: Sender<ThreadAction>,
}

impl BatchingSpanRecorder {
    pub fn start(
        collector: Box<dyn BatchCollector + Send>,
        options: BatchCollectionOptions,
    ) -> Self {
        let batch = Arc::new(Queue::default());
        let (thread_tx, thread_rx) = mpsc::channel();
        CollectThread {
            collector,
            thread_rx,
            batch_size_threshold: options.batch_size_threshold,
            batch_time_threshold: options.batch_time_threshold,
            next_collect_time: SystemTime::now() + options.batch_time_threshold,
            batch: Arc::clone(&batch),
        }
        .spawn();
        Self {
            batch,
            batch_size_threshold: options.batch_size_threshold,
            thread_tx: thread_tx,
        }
    }

    pub fn record_span(&self, span: SpanData) {
        self.batch.push(span);
        if self.batch.len() == self.batch_size_threshold {
            self.thread_tx.send(ThreadAction::Wake).ok();
        }
    }
}

/// A trait for collecting spans after they have been batched
pub trait BatchCollector {
    fn collect(&self, batch: SpanBatch);
}

impl<F: Fn(SpanBatch)> BatchCollector for F {
    fn collect(&self, spans: SpanBatch) {
        self(spans)
    }
}

pub struct BatchCollectionOptions {
    batch_size_threshold: usize,
    batch_time_threshold: Duration,
}

impl Default for BatchCollectionOptions {
    fn default() -> Self {
        Self {
            batch_size_threshold: 4096,
            batch_time_threshold: Duration::from_secs(60),
        }
    }
}

impl BatchCollectionOptions {
    pub fn with_batch_size_threshold(mut self, batch_size_threshold: usize) -> Self {
        self.batch_size_threshold = batch_size_threshold;
        self
    }

    pub fn with_batch_time_threshold(mut self, batch_time_threshold: Duration) -> Self {
        self.batch_time_threshold = batch_time_threshold;
        self
    }
}

/// A thread that collects spans from a [BatchingSpanRecorder] and sends them to a [BatchCollector]
struct CollectThread {
    collector: Box<dyn BatchCollector + Send>,
    thread_rx: Receiver<ThreadAction>,
    batch_size_threshold: usize,
    batch_time_threshold: Duration,
    next_collect_time: SystemTime,
    batch: Arc<Queue<SpanData>>,
}

impl CollectThread {
    pub fn spawn(mut self) {
        std::thread::Builder::new()
            .name("chronograph batch collector".to_owned())
            .spawn(move || self.run())
            .expect("could not spawn std thread");
    }

    pub fn run(&mut self) {
        loop {
            match self.thread_rx.recv_timeout(self.batch_time_threshold) {
                Ok(ThreadAction::Shutdown) => return,
                Ok(ThreadAction::Wake) | Err(_) => {}
            }
            if self.batch.len() >= self.batch_size_threshold
                || SystemTime::now() >= self.next_collect_time
            {
                let mut batch: Vec<SpanData> = Vec::new();
                while let Some(record) = self.batch.pop() {
                    batch.push(SpanData::clone(&record));
                }
                if !batch.is_empty() {
                    self.collector.collect(SpanBatch { spans: batch });
                }
                self.next_collect_time = SystemTime::now() + self.batch_time_threshold;
            }
        }
    }
}

impl Drop for BatchingSpanRecorder {
    fn drop(&mut self) {
        // shutdown the daemon thread when the batching span recorder is dropped
        self.thread_tx.send(ThreadAction::Shutdown).ok();
    }
}

#[derive(Debug, Clone)]
enum ThreadAction {
    Wake,
    Shutdown,
}
