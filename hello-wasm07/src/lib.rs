
extern {
    fn date_now() -> f64;
}

//use chrono::prelude::{DateTime, Utc, Local, Datelike, Timelike};

#[no_mangle]
pub fn add1(a: i32, b: i32) -> i32 {
    a + b + 17000
}

#[no_mangle]
pub fn add2(a: i32, b: i32) -> i32 {
    a + b + 22000
}

#[no_mangle]
pub fn add3(a: i32, b: i32) -> i32 {
    a + b + 33000
}

#[no_mangle]
pub fn get_timestamp() -> f64 {
    unsafe {
        date_now()
    }
}


