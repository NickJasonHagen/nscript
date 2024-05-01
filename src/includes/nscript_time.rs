
use crate::*;
use std::time::{SystemTime, UNIX_EPOCH};
pub struct Ntimer {

}

impl Ntimer {
    pub fn init() -> i64 {
        // sets a timestamp in a i64 (in nscript_fn_bindings converted to strings)
        let now = SystemTime::now();
        let duration = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
        return duration.as_millis() as i64;
    }

    pub fn diff(timerhandle: i64) -> i64 {
        // given a timestamp from init() it will give the timedifference in MS
        let now = SystemTime::now();
        let duration = now.duration_since(UNIX_EPOCH).expect("Time went backwards");
        return duration.as_millis() as i64 - timerhandle;
    }
    pub fn hours_in_ms(time: &str) -> String {
        return "".to_owned() + &(nscript_f64(&time)* nscript_f64(&"3600000")).to_string() ;
    }
    pub fn minutes_in_ms(time: &str) -> String {
        return "".to_owned() + &(nscript_f64(&time)* nscript_f64(&"60000")).to_string() ;
    }
    pub fn days_in_ms(time: &str) -> String {
        return "".to_owned() + &(nscript_f64(&time)* nscript_f64(&"86400000")).to_string() ;
    }
    pub fn weeks_in_ms(time: &str) -> String {
        return "".to_owned() + &(nscript_f64(&time)* nscript_f64(&"604800000")).to_string() ;
    }
    pub fn months_in_ms(time: &str) -> String {
        return "".to_owned() + &(nscript_f64(&time)* nscript_f64(&"2629800000")).to_string() ;
    }
    pub fn years_in_ms(time: &str) -> String {
        return "".to_owned() + &(nscript_f64(&time)* nscript_f64(&"31557600000")).to_string() ;
    }
}


