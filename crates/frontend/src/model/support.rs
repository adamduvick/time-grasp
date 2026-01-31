use time::UtcOffset;
use web_sys::js_sys;

pub fn get_timezone_offset_seconds() -> i32 {
    let date = js_sys::Date::new_0();
    let offset_minutes = date.get_timezone_offset();
    -(offset_minutes as i32) * 60
}

pub fn get_timezone_offset() -> UtcOffset {
    match UtcOffset::from_whole_seconds(get_timezone_offset_seconds()) {
        Ok(offset) => offset,
        Err(err) => {
            // TODO log to sentry
            // TODO consider bubble up the error to be handled in Fmc instead
            // then it could fallback to last-known good value instead
            leptos::logging::log!("{err:?}");
            // fallback to UTC if this fails
            UtcOffset::UTC
        }
    }
}
