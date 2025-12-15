//! Time helper types used across the model layer.
//!
//! This module provides small, strongly-typed wrappers around integer
//! millisecond timestamps and durations. Wrapping primitive types improves
//! readability and prevents accidental unit mix-ups when working with time
//! values. Both types are `sqlx::Type` transparent wrappers so they map
//! directly to integer columns in the database.
use std::ops::{Add, Sub};

use chrono::{DateTime, TimeZone, Utc};
use serde::{Deserialize, Serialize};

/// Epoch milliseconds since UNIX epoch.
///
/// A thin, transparent wrapper around `i64` representing a point in time
/// measured in milliseconds. Provides convenience conversions to/from
/// `chrono::DateTime<Utc>` and basic arithmetic with `DurationMillis`.
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", sqlx(transparent))]
pub struct EpochMillis(pub i64);

impl EpochMillis {
    /// Construct an `EpochMillis` for the current instant.
    pub fn now() -> Self {
        Self(Utc::now().timestamp_millis())
    }

    /// Convert into a `chrono::DateTime<Utc>`.
    ///
    /// Panics if the stored millisecond value is out-of-range for `chrono`.
    pub fn as_datetime(self) -> DateTime<Utc> {
        Utc.timestamp_millis_opt(self.0)
            .single()
            .expect("invalid millis")
    }
}

impl From<DateTime<Utc>> for EpochMillis {
    fn from(dt: DateTime<Utc>) -> Self {
        Self(dt.timestamp_millis())
    }
}

impl From<EpochMillis> for DateTime<Utc> {
    fn from(e: EpochMillis) -> Self {
        e.as_datetime()
    }
}

/// A signed duration measured in milliseconds.
///
/// Signed so negative durations are representable. This is a transparent
/// wrapper around `i64` and interoperates with `EpochMillis` using the
/// standard arithmetic operators implemented below.
#[cfg_attr(feature = "sqlx", derive(sqlx::Type))]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "sqlx", sqlx(transparent))]
pub struct DurationMillis(pub i64);

// time ± duration = time
impl Add<DurationMillis> for EpochMillis {
    type Output = EpochMillis;
    fn add(self, rhs: DurationMillis) -> Self::Output {
        EpochMillis(self.0 + rhs.0)
    }
}

impl Sub<DurationMillis> for EpochMillis {
    type Output = EpochMillis;
    fn sub(self, rhs: DurationMillis) -> Self::Output {
        EpochMillis(self.0 - rhs.0)
    }
}

// time - time = duration
impl Sub for EpochMillis {
    type Output = DurationMillis;
    fn sub(self, rhs: Self) -> Self::Output {
        DurationMillis(self.0 - rhs.0)
    }
}
