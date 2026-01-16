use chrono::{Offset, Utc};
use model::EpochMillis;
use reactive_stores::{PatchField, Store, StorePath};
use time::{
    Duration, OffsetDateTime, PrimitiveDateTime, UtcDateTime, UtcOffset,
    format_description::modifier::End,
};

use crate::error::Result;

/// Represents the end boundary of a timeframe
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeFrameEnd {
    /// Timeframe ends at a specific time
    EndTime(OffsetDateTime),
    /// Timeframe has a duration from the start
    Duration(Duration),
    /// Timeframe is open-ended (no end)
    Open,
}

/// A timeframe with a guaranteed start time, optional end specification, and timezone offset
#[derive(Debug, Clone, Copy, PartialEq, Eq, Store)]
pub struct TimeFrame {
    start: OffsetDateTime,
    end: TimeFrameEnd,
    offset: UtcOffset,
}

impl PatchField for TimeFrame {
    fn patch_field(&mut self, new: Self, path: &StorePath, notify: &mut dyn FnMut(&StorePath)) {
        leptos::logging::log!("TimeFrame::patch_field called: {:?}", new);
        if new != *self {
            leptos::logging::log!("TimeFrame::patch_field change detected");
            *self = new;
            leptos::logging::log!("TimeFrame::patch_field patch complete");
            notify(path);
            leptos::logging::log!("TimeFrame::patch_field patch notified");
        } else {
            leptos::logging::log!("TimeFrame::patch_field no change");
        }
    }
}

impl TimeFrame {
    /// Creates a new timeframe with the given start, end specification, and offset
    pub fn new(start: OffsetDateTime, end: TimeFrameEnd, offset: UtcOffset) -> Self {
        Self { start, end, offset }
    }

    /// Convenience converter from `EpochMillis` start and end times.
    pub fn with_utc_start_and_end(start: EpochMillis, end: Option<EpochMillis>) -> Result<Self> {
        let start: UtcDateTime = start.try_into()?;
        let end: Option<UtcDateTime> = match end {
            Some(end) => Some(end.try_into()?),
            None => None,
        };

        Ok(Self::new(
            start.into(),
            match end {
                Some(end) => TimeFrameEnd::EndTime(end.into()),
                None => TimeFrameEnd::Open,
            },
            UtcOffset::UTC,
        ))
    }

    pub fn get_start_time(&self) -> OffsetDateTime {
        self.start.to_offset(self.offset)
    }

    pub fn get_end_time(&self) -> Option<OffsetDateTime> {
        match self.end {
            TimeFrameEnd::EndTime(end) => Some(end.to_offset(self.offset)),
            TimeFrameEnd::Duration(duration) => self
                .start
                .checked_add(duration)
                .map(|dt| dt.to_offset(self.offset)),
            TimeFrameEnd::Open => None,
        }
    }

    pub fn get_duration(&self) -> Option<Duration> {
        match self.end {
            TimeFrameEnd::EndTime(end) => Some(end - self.start),
            TimeFrameEnd::Duration(duration) => Some(duration),
            TimeFrameEnd::Open => None,
        }
    }

    pub fn get_utc_start_time(&self) -> UtcDateTime {
        self.start.to_offset(self.offset).into()
    }

    pub fn get_utc_end_time(&self) -> Option<UtcDateTime> {
        match self.get_end_time() {
            Some(dt) => Some(dt.to_offset(self.offset).into()),
            None => None,
        }
    }

    pub fn assume_start_time(&mut self, start: PrimitiveDateTime) {
        self.start = start.assume_offset(self.offset);
    }

    pub fn assume_end_time(&mut self, end: PrimitiveDateTime) {
        self.end = TimeFrameEnd::EndTime(end.assume_offset(self.offset));
    }

    pub fn assume_duration(&mut self, duration: Duration) {
        self.end = TimeFrameEnd::Duration(duration);
    }

    pub fn assume_open(&mut self) {
        self.end = TimeFrameEnd::Open;
    }

    pub fn assume_offset(&mut self, offset: UtcOffset) {
        leptos::logging::log!("TimeFrame::assume_offset: {offset}");
        self.offset = offset;
    }

    pub fn with_offset(&self, offset: UtcOffset) -> Self {
        leptos::logging::log!("changed!TimeFrame::with_offset: {offset}");
        Self {
            start: self.start,
            end: self.end,
            offset,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
