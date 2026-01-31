use chrono::{Offset, Utc};
use model::EpochMillis;
use reactive_stores::{PatchField, Store, StorePath};
use time::{
    Date, Duration, OffsetDateTime, PrimitiveDateTime, Time, UtcDateTime, UtcOffset,
    format_description::modifier::End,
};

use crate::error::Result;

/// Represents the end boundary of a timeframe
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimeFrameEnd {
    /// Timeframe ends at a specific time
    EndTime(UtcDateTime),
    /// Timeframe has a duration from the start
    Duration(Duration),
    /// Timeframe is open-ended (no end)
    Open,
}

/// A timeframe with a guaranteed start time, optional end specification, and timezone offset
#[derive(Debug, Clone, Copy, PartialEq, Eq, Store)]
pub struct TimeFrame {
    start: UtcDateTime,
    end: TimeFrameEnd,
}

impl PatchField for TimeFrame {
    fn patch_field(&mut self, new: Self, path: &StorePath, notify: &mut dyn FnMut(&StorePath)) {
        if new != *self {
            *self = new;
            notify(path);
        } else {
        }
    }
}

impl TimeFrame {
    /// Creates a new timeframe with the given start, end specification, and offset
    pub fn new(start: UtcDateTime, end: TimeFrameEnd) -> Self {
        Self { start, end }
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
        ))
    }
    // #[deprecated]
    // pub fn get_start_datetime(&self) -> OffsetDateTime {
    //     self.start.to_offset(self.offset)
    // }
    // #[deprecated]
    // pub fn get_end_datetime(&self) -> Option<OffsetDateTime> {
    //     match self.end {
    //         TimeFrameEnd::EndTime(end) => Some(end.to_offset(self.offset)),
    //         TimeFrameEnd::Duration(duration) => self
    //             .start
    //             .checked_add(duration)
    //             .map(|dt| dt.to_offset(self.offset)),
    //         TimeFrameEnd::Open => None,
    //     }
    // }
    // #[deprecated]
    // pub fn get_duration(&self) -> Option<Duration> {
    //     match self.end {
    //         TimeFrameEnd::EndTime(end) => Some(end - self.start),
    //         TimeFrameEnd::Duration(duration) => Some(duration),
    //         TimeFrameEnd::Open => None,
    //     }
    // }
    // #[deprecated]
    // pub fn get_utc_start_time(&self) -> UtcDateTime {
    //     self.start.to_offset(self.offset).into()
    // }
    // #[deprecated]
    // pub fn get_utc_end_time(&self) -> Option<UtcDateTime> {
    //     match self.get_end_datetime() {
    //         Some(dt) => Some(dt.to_offset(self.offset).into()),
    //         None => None,
    //     }
    // }
    // #[deprecated]
    // pub fn assume_start_time(&mut self, start: PrimitiveDateTime) {
    //     self.start = start.assume_offset(self.offset);
    // }
    // #[deprecated]
    // pub fn assume_end_time(&mut self, end: PrimitiveDateTime) {
    //     self.end = TimeFrameEnd::EndTime(end.assume_offset(self.offset));
    // }
    // #[deprecated]
    // pub fn assume_duration(&mut self, duration: Duration) {
    //     self.end = TimeFrameEnd::Duration(duration);
    // }
    // #[deprecated]
    // pub fn assume_open(&mut self) {
    //     self.end = TimeFrameEnd::Open;
    // }

    // #[deprecated]
    // pub fn assume_offset(&mut self, offset: UtcOffset) {
    //     leptos::logging::log!("TimeFrame::assume_offset: {offset}");
    //     self.offset = offset;
    // }

    // #[deprecated]
    // pub fn with_offset(&self, offset: UtcOffset) -> Self {
    //     leptos::logging::log!("changed!TimeFrame::with_offset: {offset}");
    //     Self {
    //         start: self.start,
    //         end: self.end,
    //         offset,
    //     }
    // }

    pub fn get_utc_start_datetime(&self) -> UtcDateTime {
        self.start
    }

    pub fn get_start_datetime(&self, offset: UtcOffset) -> OffsetDateTime {
        self.start.to_offset(offset)
    }

    pub fn get_start_date(&self, offset: UtcOffset) -> Date {
        self.get_start_datetime(offset).date()
    }

    pub fn get_start_time(&self, offset: UtcOffset) -> Time {
        self.get_start_datetime(offset).time()
    }

    pub fn get_utc_end_datetime(&self) -> Option<UtcDateTime> {
        match self.end {
            TimeFrameEnd::EndTime(dt) => Some(dt),
            TimeFrameEnd::Duration(duration) => self.start.checked_add(duration).map(|dt| dt),
            TimeFrameEnd::Open => None,
        }
    }

    pub fn get_end_datetime(&self, offset: UtcOffset) -> Option<OffsetDateTime> {
        self.get_utc_end_datetime().map(|dt| dt.to_offset(offset))
    }

    pub fn get_end_date(&self, offset: UtcOffset) -> Option<Date> {
        self.get_end_datetime(offset).map(|dt| dt.date())
    }

    pub fn get_end_time(&self, offset: UtcOffset) -> Option<Time> {
        self.get_end_datetime(offset).map(|dt| dt.time())
    }

    pub fn get_duration(&self) -> Option<Duration> {
        match self.end {
            TimeFrameEnd::EndTime(dt) => Some(dt - self.start),
            TimeFrameEnd::Duration(duration) => Some(duration),
            TimeFrameEnd::Open => None,
        }
    }

    pub fn set_start_date(&mut self, date: Date, offset: UtcOffset) {
        self.start = self.start.to_offset(offset).replace_date(date).into();
    }

    pub fn set_start_time(&mut self, time: Time, offset: UtcOffset) {
        self.start = self.start.to_offset(offset).replace_time(time).into();
    }

    pub fn set_end_date(&mut self, date: Date, offset: UtcOffset) {
        // if no time is previously set, assume end time is same as start time
        let fallback_time = self.get_start_time(offset);
        let new_end_time = match (self.get_end_date(offset), self.get_end_time(offset)) {
            (None, None) => OffsetDateTime::new_in_offset(date, fallback_time, offset),
            (Some(_), Some(time)) => OffsetDateTime::new_in_offset(date, time, offset),
            // unreachable, since date cannot be none while time is some
            (None, Some(time)) => OffsetDateTime::new_in_offset(date, time, offset),
            // unreachable, since time cannot be none while date is some
            (Some(_), None) => OffsetDateTime::new_in_offset(date, fallback_time, offset),
        };
        self.end = TimeFrameEnd::EndTime(new_end_time.into());
    }

    pub fn set_end_time(&mut self, time: Time, offset: UtcOffset) {
        // if no date is previously set, assume end date is same as start date
        let fallback_date = self.get_start_date(offset);
        let new_end_time = match (self.get_end_date(offset), self.get_end_time(offset)) {
            (None, None) => OffsetDateTime::new_in_offset(fallback_date, time, offset),
            (Some(date), Some(_)) => OffsetDateTime::new_in_offset(date, time, offset),
            // unreachable, since date cannot be none while time is some
            (None, Some(_)) => OffsetDateTime::new_in_offset(fallback_date, time, offset),
            // unreachable, since time cannot be none while date is some
            (Some(date), None) => OffsetDateTime::new_in_offset(date, time, offset),
        };
        self.end = TimeFrameEnd::EndTime(new_end_time.into());
    }

    pub fn set_duration(&mut self, duration: Duration) {
        self.end = TimeFrameEnd::Duration(duration);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
