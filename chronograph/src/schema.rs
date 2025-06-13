//! Serialization and deserialization of chronograph data, utilizing rkyv for fast serialization and deserialization.

use std::{hash::Hasher, time::SystemTime};

use rkyv::util::AlignedVec;
use zwohash::ZwoHasher;

#[derive(Debug, Clone, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub struct SpanBatch {
    pub spans: Vec<SpanData>,
}

#[derive(Debug, Clone, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub struct SpanData {
    pub span_id: u64,
    pub start_unix_time: i64,
    pub start_instant: u64,
    pub end_instant: u64,
    pub records: Vec<RecordData>,
}

#[derive(Debug, Clone, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub struct RecordData {
    pub datapoint_id: DatapointId,
    pub value: RecordValue,
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize,
)]
pub struct DatapointId {
    pub value: u64,
}

#[derive(Debug, Clone, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize)]
pub enum RecordValue {
    /// Monotonic instant (accurate nanosecond timer elapsed from when the Cronograph was started)
    Instant(u64),
    /// Unix timestamp (as nanoseconds since epoch)
    UnixTime(i64),
    /// An arbitrary string value formatted as UTF-8
    Utf8String(String),
    /// An arbitrary i32 value
    I32(i32),
    /// An arbitrary i64 value
    I64(i64),
    /// An arbitrary i128 value
    I128(i128),
    /// An arbitrary u32 value
    U32(u32),
    /// An arbitrary u64 value
    U64(u64),
    /// An arbitrary u128 value
    U128(u128),
    /// An arbitrary f64 value
    F32(f32),
    /// An arbitrary f64 value
    F64(f64),
}

impl From<&SpanBatch> for AlignedVec {
    fn from(value: &SpanBatch) -> Self {
        rkyv::to_bytes::<rkyv::rancor::Error>(value).unwrap_or_default()
    }
}

impl From<SpanBatch> for AlignedVec {
    fn from(value: SpanBatch) -> Self {
        rkyv::to_bytes::<rkyv::rancor::Error>(&value).unwrap_or_default()
    }
}

impl From<&SpanBatch> for Vec<u8> {
    fn from(value: &SpanBatch) -> Self {
        rkyv::to_bytes::<rkyv::rancor::Error>(value)
            .unwrap_or_default()
            .into_vec()
    }
}

impl From<SpanBatch> for Vec<u8> {
    fn from(value: SpanBatch) -> Self {
        rkyv::to_bytes::<rkyv::rancor::Error>(&value)
            .unwrap_or_default()
            .into_vec()
    }
}

impl TryFrom<&[u8]> for SpanBatch {
    type Error = rkyv::rancor::Error;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        rkyv::from_bytes::<Self, rkyv::rancor::Error>(value)
    }
}

impl From<&SpanData> for AlignedVec {
    fn from(value: &SpanData) -> Self {
        rkyv::to_bytes::<rkyv::rancor::Error>(value).unwrap_or_default()
    }
}

impl From<SpanData> for AlignedVec {
    fn from(value: SpanData) -> Self {
        rkyv::to_bytes::<rkyv::rancor::Error>(&value).unwrap_or_default()
    }
}

impl From<&SpanData> for Vec<u8> {
    fn from(value: &SpanData) -> Self {
        rkyv::to_bytes::<rkyv::rancor::Error>(value)
            .unwrap_or_default()
            .into_vec()
    }
}

impl From<SpanData> for Vec<u8> {
    fn from(value: SpanData) -> Self {
        rkyv::to_bytes::<rkyv::rancor::Error>(&value)
            .unwrap_or_default()
            .into_vec()
    }
}

impl TryFrom<&[u8]> for SpanData {
    type Error = rkyv::rancor::Error;
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        rkyv::from_bytes::<Self, rkyv::rancor::Error>(value)
    }
}

impl From<u64> for DatapointId {
    fn from(value: u64) -> Self {
        Self { value }
    }
}

impl From<&str> for DatapointId {
    fn from(value: &str) -> Self {
        let mut hasher = ZwoHasher::default();
        hasher.write(value.as_bytes());
        Self {
            value: hasher.finish(),
        }
    }
}

impl From<SystemTime> for RecordValue {
    fn from(value: SystemTime) -> Self {
        Self::UnixTime(
            value
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap_or_default()
                .as_nanos() as i64,
        )
    }
}

impl From<usize> for RecordValue {
    fn from(value: usize) -> Self {
        Self::U64(value as u64)
    }
}

impl From<String> for RecordValue {
    fn from(value: String) -> Self {
        Self::Utf8String(value)
    }
}

impl From<i32> for RecordValue {
    fn from(value: i32) -> Self {
        Self::I32(value)
    }
}

impl From<i64> for RecordValue {
    fn from(value: i64) -> Self {
        Self::I64(value)
    }
}

impl From<i128> for RecordValue {
    fn from(value: i128) -> Self {
        Self::I128(value)
    }
}

impl From<u32> for RecordValue {
    fn from(value: u32) -> Self {
        Self::U32(value)
    }
}

impl From<u64> for RecordValue {
    fn from(value: u64) -> Self {
        Self::U64(value)
    }
}

impl From<u128> for RecordValue {
    fn from(value: u128) -> Self {
        Self::U128(value)
    }
}

impl From<f32> for RecordValue {
    fn from(value: f32) -> Self {
        Self::F32(value)
    }
}

impl From<f64> for RecordValue {
    fn from(value: f64) -> Self {
        Self::F64(value)
    }
}
