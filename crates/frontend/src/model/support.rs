use time::UtcOffset;
use web_sys::js_sys;

pub fn get_timezone_offset_seconds() -> i32 {
    let date = js_sys::Date::new_0();
    let offset_minutes = date.get_timezone_offset();
    -(offset_minutes as i32) * 60
}

pub fn get_timezone_offset() -> UtcOffset {
    // TODO learn if this can ever fail if we get the offset directly from the browser API
    UtcOffset::from_whole_seconds(get_timezone_offset_seconds()).unwrap()
}
